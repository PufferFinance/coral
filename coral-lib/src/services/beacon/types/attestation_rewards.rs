use serde::Deserialize;

use super::BeaconResponse;

#[derive(Clone, Debug, Deserialize)]
pub struct IdealRewardResponse {
    pub effective_balance: String,
    pub head: String,
    pub target: String,
    pub source: String,
    pub inclusion_delay: Option<String>,
    pub inactivity: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TotalRewardResponse {
    pub validator_index: String,
    pub head: String,
    pub target: String,
    pub source: String,
    pub inclusion_delay: Option<String>,
    pub inactivity: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AttestationRewardsDataResponse {
    pub ideal_rewards: Vec<IdealRewardResponse>,
    pub total_rewards: Vec<TotalRewardResponse>,
}

pub type AttestationRewardsResponse = BeaconResponse<AttestationRewardsDataResponse>;
