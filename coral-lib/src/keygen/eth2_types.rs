use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::{typenum, FixedVector};
use tree_hash_derive::TreeHash;

/// Basic types
pub type Bytes4 = [u8; 4];
pub type Bytes32 = [u8; 32];
pub type Bytes48 = FixedVector<u8, typenum::U48>;
pub type Bytes96 = FixedVector<u8, typenum::U96>;
pub type Root = Bytes32;
pub type BLSSignature = Bytes96;
pub type BLSPubkey = Bytes48;
pub type Version = Bytes4;
pub type Gwei = u64;
pub type DomainType = Bytes4;
pub type Domain = Bytes32;

/// Domain constants
pub const DOMAIN_DEPOSIT: DomainType = [3_u8, 0_u8, 0_u8, 0_u8]; // '0x03000000'
pub const GENESIS_FORK_VERSION: Version = [0_u8, 0_u8, 0_u8, 0_u8]; // '0x00000000'

/// Deposit message for signing
#[derive(Debug, Clone, Encode, Decode, TreeHash, Serialize, Deserialize)]
pub struct DepositMessage {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: Bytes32,
    pub amount: Gwei,
}

/// Full deposit data including signature
#[derive(Debug, Clone, Encode, Decode, TreeHash, Serialize, Deserialize)]
pub struct DepositData {
    pub pubkey: BLSPubkey,
    pub withdrawal_credentials: Bytes32,
    pub amount: Gwei,
    pub signature: BLSSignature,
}

/// Fork data for computing domain
#[derive(Debug, Clone, Encode, Decode, TreeHash, Serialize, Deserialize)]
pub struct ForkData {
    pub current_version: Version,
    pub genesis_validators_root: Root,
}

/// Signing data wrapper
#[derive(Debug, Clone, Encode, Decode, TreeHash, Serialize, Deserialize)]
pub struct SigningData {
    pub object_root: Root,
    pub domain: Domain,
}

