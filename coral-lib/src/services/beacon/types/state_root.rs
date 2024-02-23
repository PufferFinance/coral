use ethers::types::{H256, U256};
use serde::{Deserialize, Serialize};

use crate::utils::serialize::u256_serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StateId {
    #[serde(rename = "head")]
    Head,
    #[serde(rename = "genesis")]
    Genesis,
    #[serde(rename = "finalized")]
    Finalized,
    #[serde(rename = "justified")]
    Justified,
    #[serde(untagged, serialize_with = "u256_serialize")]
    Slot(U256),
    #[serde(untagged)]
    StateRoot(H256),
}

impl std::fmt::Display for StateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Head => f.write_str("head"),
            Self::Genesis => f.write_str("genesis"),
            Self::Finalized => f.write_str("finalized"),
            Self::Justified => f.write_str("justified"),
            Self::Slot(s) => f.write_str(&format!("{}", s)),
            Self::StateRoot(s) => f.write_str(&format!("0x{:x}", s)),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct StateRootData {
    pub root: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StateRootResponse {
    pub execution_optimistic: bool,
    pub finalized: bool,
    pub data: StateRootData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BlockId {
    #[serde(rename = "head")]
    Head,
    #[serde(rename = "genesis")]
    Genesis,
    #[serde(rename = "finalized")]
    Finalized,
    #[serde(rename = "justified")]
    Justified,
    #[serde(untagged, serialize_with = "u256_serialize")]
    Slot(U256),
    #[serde(untagged)]
    BlockRoot(String),
}

impl std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Head => f.write_str("head"),
            Self::Genesis => f.write_str("genesis"),
            Self::Finalized => f.write_str("finalized"),
            Self::Justified => f.write_str("justified"),
            Self::Slot(s) => f.write_str(&s.to_string()),
            Self::BlockRoot(s) => f.write_str(&s.to_string()),
        }
    }
}
