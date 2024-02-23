use ethers::types::U256;
use serde::Deserialize;

use crate::utils::serialize::{string_to_u256_base10, string_to_u64};

use super::BeaconResponse;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ValidatorId {
    Pubkey(String),
    Index(u64),
}

impl std::fmt::Display for ValidatorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pubkey(key) => f.write_str(key),
            Self::Index(index) => f.write_str(&format!("{}", *index)),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ValidatorStatus {
    #[serde(rename = "pending_initialized")]
    PendingInitialized,
    #[serde(rename = "pending_queued")]
    PendingQueued,
    #[serde(rename = "active_ongoing")]
    ActiveOngoing,
    #[serde(rename = "active_exiting")]
    ActiveExiting,
    #[serde(rename = "active_slashed")]
    ActiveSlashed,
    #[serde(rename = "exited_unslashed")]
    ExitedUnslashed,
    #[serde(rename = "exited_slashed")]
    ExitedSlashed,
    #[serde(rename = "withdrawal_possible")]
    WithdrawalPossible,
    #[serde(rename = "withdrawal_done")]
    WithdrawalDone,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ValidatorStats {
    pub pubkey: String,
    pub withdrawal_credentials: String,
    #[serde(deserialize_with = "string_to_u256_base10")]
    pub effective_balance: U256,
    pub slashed: bool,
    #[serde(deserialize_with = "string_to_u64")]
    pub activation_eligibility_epoch: u64,
    #[serde(deserialize_with = "string_to_u64")]
    pub activation_epoch: u64,
    #[serde(deserialize_with = "string_to_u64")]
    pub exit_epoch: u64,
    #[serde(deserialize_with = "string_to_u64")]
    pub withdrawable_epoch: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ValidatorData {
    #[serde(deserialize_with = "string_to_u64")]
    pub index: u64,
    #[serde(deserialize_with = "string_to_u256_base10")]
    pub balance: U256,
    pub status: ValidatorStatus,
    pub validator: ValidatorStats,
}

pub type ValidatorResponse = BeaconResponse<ValidatorData>;
pub type ValidatorListResponse = BeaconResponse<Vec<ValidatorData>>;
