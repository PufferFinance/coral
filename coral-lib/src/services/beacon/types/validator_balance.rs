use ethers::types::U256;
use serde::Deserialize;

use super::BeaconResponse;

#[derive(Clone, Debug, Deserialize)]
pub struct ValidatorBalance {
    pub id: u64,
    pub balance: U256,
}

pub type ValidatorBalancesResponse = BeaconResponse<Vec<ValidatorBalance>>;
