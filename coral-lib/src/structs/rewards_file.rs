use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RewardsRawFile {
    pub metadata: Metadata,
    pub node_operators: HashMap<String, NodeOperator>,
    pub merkle_root: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub total_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeOperator {
    pub total: String,
    pub validator_amounts: Vec<ValidatorAmount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorAmount {
    pub beacon_index: u64,
    pub earned_amount: String,
}
