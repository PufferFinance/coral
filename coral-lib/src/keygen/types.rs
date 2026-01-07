use serde::{Deserialize, Serialize};

use super::eth2_types::Version;

/// Input payload for generating a fresh BLS key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestFreshBlsKeyPayload {
    pub withdrawal_credentials: [u8; 32],
    pub fork_version: Version,
}

/// Output payload containing the generated BLS key information
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlsKeygenPayload {
    pub bls_pub_key: String,
    pub signature: String,
    pub deposit_data_root: String,
    pub withdrawal_credentials: String,
    pub fork_version: Version,
}
