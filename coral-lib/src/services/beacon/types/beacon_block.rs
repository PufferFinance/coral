use ethers::types::U256;
use serde::Deserialize;

use crate::utils::serialize::{string_to_u256_base10, string_to_u64};

#[derive(Clone, Debug, Deserialize)]
pub struct BeaconWithdrawal {
    pub index: String,
    #[serde(deserialize_with = "string_to_u64")]
    pub validator_index: u64,
    pub address: String,
    #[serde(deserialize_with = "string_to_u256_base10")]
    pub amount: U256,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExecutionPayload {
    pub parent_hash: String,
    pub fee_recipient: String,
    pub state_root: String,
    pub receipts_root: String,
    pub logs_bloom: String,
    pub prev_randao: String,
    pub block_number: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub timestamp: String,
    pub extra_data: String,
    pub base_fee_per_gas: String,
    pub block_hash: String,
    pub transactions: Vec<String>,
    pub withdrawals: Vec<BeaconWithdrawal>,
    #[serde(skip_deserializing)]
    pub bls_to_execution_changes: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BeaconEth1BlockBody {
    pub deposit_root: String,
    pub deposit_count: String,
    pub block_hash: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BeaconBlockBody {
    pub randao_reveal: String,
    pub eth1_data: BeaconEth1BlockBody,
    pub graffiti: String,
    #[serde(skip_deserializing)]
    pub proposer_slashings: Vec<serde_json::Value>,
    #[serde(skip_deserializing)]
    pub attester_slashings: Vec<serde_json::Value>,
    #[serde(skip_deserializing)]
    pub attestations: Vec<serde_json::Value>,
    #[serde(skip_deserializing)]
    pub deposits: Vec<serde_json::Value>,
    #[serde(skip_deserializing)]
    pub voluntary_exits: Vec<serde_json::Value>,
    #[serde(skip_deserializing)]
    pub sync_aggregate: Option<Vec<serde_json::Value>>,
    pub execution_payload: ExecutionPayload,
    #[serde(skip_deserializing)]
    pub bls_to_execution_changes: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockMessageData {
    #[serde(deserialize_with = "string_to_u256_base10")]
    pub slot: U256,
    pub proposer_index: String,
    pub parent_root: String,
    pub state_root: String,
    pub body: BeaconBlockBody,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockMessage {
    pub message: BlockMessageData,
    pub signature: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockResponse {
    pub execution_optimistic: Option<bool>,
    pub finalized: Option<bool>,
    pub data: BlockMessage,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockRoot {
    pub root: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlockRootResponse {
    pub execution_optimistic: Option<bool>,
    pub finalized: Option<bool>,
    pub data: BlockRoot,
}
