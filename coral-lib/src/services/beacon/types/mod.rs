mod attestation_rewards;
mod beacon_block;
mod genesis;
mod state_root;
mod validator;
mod validator_balance;

#[cfg(test)]
mod state_root_test;

pub use attestation_rewards::*;
pub use beacon_block::*;
pub use genesis::*;
pub use state_root::*;
pub use validator::*;
pub use validator_balance::*;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BeaconResponse<T> {
    pub finalized: Option<bool>,
    pub data: T,
}
