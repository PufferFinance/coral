pub mod types;

mod attestation_rewards;
mod fetch_block;
mod fetch_genesis;
mod fetch_puffer_validator_stats;
mod fetch_state_root;
mod fetch_validator;
mod fetch_validators;
mod submit_exit;

pub mod client;
pub mod mock_client;

pub use self::attestation_rewards::*;
pub use self::fetch_block::*;
pub use self::fetch_genesis::*;
pub use self::fetch_puffer_validator_stats::*;
pub use self::fetch_state_root::*;
pub use self::fetch_validator::*;
pub use self::fetch_validators::*;
pub use self::submit_exit::*;

pub const SLOTS_PER_EPOCH: u64 = 32;

#[inline]
pub fn is_active_validator(data: &types::ValidatorData, epoch: u64) -> bool {
    data.validator.activation_epoch <= epoch && epoch < data.validator.exit_epoch
}

#[inline]
pub fn is_full_withdrawal(data: &types::ValidatorData, epoch: u64) -> bool {
    data.validator.exit_epoch <= epoch && data.validator.withdrawable_epoch <= epoch
}
