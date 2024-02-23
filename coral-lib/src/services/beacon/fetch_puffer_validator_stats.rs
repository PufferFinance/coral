use std::collections::HashMap;

use ethers::prelude::*;

use crate::error::AppServerResult;
use crate::services::beacon::types::ValidatorData;
use crate::services::beacon::SLOTS_PER_EPOCH;


use super::client::BeaconClientTrait;
use super::types::{BlockId, ValidatorStatus};
use super::{is_active_validator, is_full_withdrawal};

#[derive(Clone, Debug)]
pub struct NodeValidator {
    pub pub_key: String,
    pub validator_id: u64,
}

#[derive(Clone, Debug, Default)]
pub struct ValidatorFullWithdrawal {
    pub amount: U256,
    pub was_slashed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ValidatorConsensusReward {
    pub amount: U256,
}

#[derive(Clone, Debug, Default)]
pub struct ValidatorReward {
    pub pubkey: String,
    pub consensus_rewards: ValidatorConsensusReward,
    pub full_withdrawal: ValidatorFullWithdrawal,
}

#[derive(Clone, Debug, Default)]
pub struct RewardBlock {
    pub block_number: String,
    pub block_root: String,
}

#[derive(Clone, Debug, Default)]
pub struct PufferValidatorStats {
    pub start_block: RewardBlock,
    pub end_block: RewardBlock,
    pub validators: HashMap<u64, ValidatorReward>,
}

pub async fn fetch_puffer_validator_stats(
    beacon_client: &impl BeaconClientTrait,
    validators_by_index: &HashMap<u64, ValidatorData>,
    last_post_block: U256,
    block_root: &str,
) -> AppServerResult<PufferValidatorStats> {
    let eth_32: U256 = U256::from(32).saturating_mul(U256::exp10(9));

    let last_reward_block_str = format!("{}", last_post_block);

    let mut validator_rewards = PufferValidatorStats::default();

    let mut end_block: Option<RewardBlock> = None;
    let start_block: Option<RewardBlock>;

    let mut block_root = block_root.to_string();
    loop {
        let curr_block = beacon_client
            .fetch_block(BlockId::BlockRoot(block_root.to_string()))
            .await?;
        if end_block.is_none() {
            end_block = Some(RewardBlock {
                block_number: curr_block
                    .data
                    .message
                    .body
                    .execution_payload
                    .block_number
                    .clone(),
                block_root: block_root.clone(),
            });
        }

        tracing::debug!(
            "BLOCK: {} {} {block_root}",
            last_reward_block_str,
            curr_block.data.message.body.execution_payload.block_number
        );

        block_root = curr_block.data.message.parent_root;
        let (epoch, _) = curr_block
            .data
            .message
            .slot
            .div_mod(U256::from(SLOTS_PER_EPOCH));

        let withdrawals = curr_block.data.message.body.execution_payload.withdrawals;
        for withdrawal in withdrawals {
            let validator_index = withdrawal.validator_index;
            if let Some(validator_data) = validators_by_index.get(&validator_index) {
                let v_fully_withdrawn = is_full_withdrawal(validator_data, epoch.as_u64());
                let entry = validator_rewards
                    .validators
                    .entry(withdrawal.validator_index)
                    .or_insert(ValidatorReward::default());

                entry.pubkey = validator_data.validator.pubkey.to_string();
                if v_fully_withdrawn {
                    if withdrawal.amount > eth_32 {
                        entry.full_withdrawal.amount = eth_32;
                        entry.consensus_rewards.amount += withdrawal.amount - eth_32;
                    } else {
                        entry.full_withdrawal.amount = withdrawal.amount;
                    }
                    match validator_data.status {
                        ValidatorStatus::ActiveSlashed | ValidatorStatus::ExitedSlashed => {
                            entry.full_withdrawal.was_slashed = true;
                        }
                        _ => {
                            entry.full_withdrawal.was_slashed = false;
                        }
                    }
                } else {
                    entry.consensus_rewards.amount += withdrawal.amount;
                }
            }
        }
        if curr_block.data.message.body.execution_payload.block_number <= last_reward_block_str {
            start_block = Some(RewardBlock {
                block_number: curr_block
                    .data
                    .message
                    .body
                    .execution_payload
                    .block_number
                    .clone(),
                block_root: block_root.clone(),
            });
            break;
        }
    }

    if let Some(block) = start_block {
        validator_rewards.start_block = block;
    }
    if let Some(block) = end_block {
        validator_rewards.end_block = block;
    }

    // convert GWEI to WEI
    validator_rewards
        .validators
        .iter_mut()
        .for_each(|(_, reward)| {
            reward.consensus_rewards.amount = reward
                .consensus_rewards
                .amount
                .saturating_mul(U256::exp10(9));
            reward.full_withdrawal.amount =
                reward.full_withdrawal.amount.saturating_mul(U256::exp10(9));
        });
    Ok(validator_rewards)
}

/// Given an epoch, count the number of valid validators in
/// validators_by_index.
pub fn count_active_validators(
    validators_by_index: &HashMap<u64, ValidatorData>,
    end_epoch: u64,
) -> u64 {
    validators_by_index
        .values()
        .filter(|v| is_active_validator(v, end_epoch))
        .count() as u64
}

/// Calculates Puffer Validator balances with just their
/// raw balance on the beacon chain.
pub fn calculate_puffer_validator_beacon_balances(
    validators_by_index: &HashMap<u64, ValidatorData>,
    end_epoch: U256,
) -> U256 {
    let gwei_32 = U256::from(32).saturating_mul(U256::exp10(9));
    let mut validator_balance = U256::zero();
    for validator in validators_by_index.values() {
        let is_active = is_active_validator(validator, end_epoch.as_u64());
        // Edge case where validator just registered and is waiting to
        // start validating
        #[allow(clippy::if_same_then_else)]
        if validator.status == ValidatorStatus::PendingQueued {
            validator_balance += gwei_32;
        } else if is_active && validator.balance == U256::zero() {
            validator_balance += gwei_32;
        } else if is_active {
            validator_balance += validator.balance;
        }
    }
    validator_balance.saturating_mul(U256::exp10(9))
}
