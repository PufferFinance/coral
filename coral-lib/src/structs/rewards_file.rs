use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rewards data from json file
#[derive(Serialize, Deserialize, Debug)]
pub struct RewardsRawFile {
    /// Metadata including start and end epochs and the total rewards amount.
    pub metadata: Metadata,
    /// Mapping of node operator addresses to their corresponding rewards data.
    pub node_operators: HashMap<String, NodeOperator>,
    /// Merkle root of the rewards data, posted on-chain, that will be verified.
    pub merkle_root: String,
}

/// Rewards metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    /// The start epoch - block number - from which rewards data begin to be collected.
    pub start_epoch: u64,
    /// The end epoch - block number - at which rewards data collection stops.
    pub end_epoch: u64,
    /// The total amount of rewards distributed between the two epochs.
    #[serde(deserialize_with = "deserialize_u256_from_number")]
    pub total_amount: U256,
    /// List of eigenpod addresses
    pub eigenpod_addresses: Vec<String>,
}

/// Detailed rewards data for a single node operator.
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeOperator {
    /// Total amount of rewards earned by this node operator.
    #[serde(deserialize_with = "deserialize_u256_from_number")]
    pub total: U256,
    /// A list of individual validator rewards, each associated with a beacon index and earned amount.
    pub validator_amounts: Vec<ValidatorAmount>,
}

/// Rewards earned by a single validator within a specified period.
#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorAmount {
    /// The index of the validator within the beacon chain.
    pub beacon_index: u64,
    /// The amount of rewards earned by the validator during the specified period.
    #[serde(deserialize_with = "deserialize_u256_from_number")]
    pub earned_amount: U256,
}

/// Deserialization function for U256 from number in JSON
/// We dont expect number to be too large for u64
/// As total rewards pushed in GWEI should not be above 2^64
fn deserialize_u256_from_number<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(num) => {
            if let Some(n) = num.as_u64() {
                Ok(U256::from(n))
            } else {
                Err(serde::de::Error::custom("Number too large for u64"))
            }
        }
        _ => Err(serde::de::Error::custom("Expected a number")),
    }
}
