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
    pub total_amount: String,
}

/// Detailed rewards data for a single node operator.
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeOperator {
    /// Total amount of rewards earned by this node operator.
    pub total: String,
    /// A list of individual validator rewards, each associated with a beacon index and earned amount.
    pub validator_amounts: Vec<ValidatorAmount>,
}

/// Rewards earned by a single validator within a specified period.
#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorAmount {
    /// The index of the validator within the beacon chain.
    pub beacon_index: u64,
    /// The amount of rewards earned by the validator during the specified period.
    pub earned_amount: String,
}
