pub mod bls_keys;
pub mod constants;
pub mod crypto;
pub mod eth2_signing;
pub mod eth2_types;
pub mod key_management;
pub mod simple_bls_keygen;
pub mod types;

pub use types::{AttestFreshBlsKeyPayload, BlsKeygenPayload};

use anyhow::Result;

/// Generates a BLS keystore and returns the keygen payload.
/// This is the main entry point for BLS key generation.
pub fn generate_bls_keystore_handler(
    keygen_payload: AttestFreshBlsKeyPayload,
    keystore_password: &str,
) -> Result<BlsKeygenPayload> {
    generate_bls_keystore(
        keygen_payload.withdrawal_credentials,
        keystore_password,
        keygen_payload.fork_version,
    )
}

fn generate_bls_keystore(
    withdrawal_credentials: [u8; 32],
    password: &str,
    fork_version: eth2_types::Version,
) -> Result<BlsKeygenPayload> {
    // Generate a new BLS key pair
    let (sk, pk) = bls_keys::generate_bls_keypair();

    // Save validator private key to encrypted keystore
    bls_keys::save_bls_keystore(&sk, password)?;

    // Sign DepositMessage to deposit 32 ETH to beacon deposit contract
    let (signature, deposit_data_root) = eth2_signing::sign_full_deposit(
        &sk,
        withdrawal_credentials,
        fork_version,
    )?;

    Ok(BlsKeygenPayload {
        bls_pub_key: pk.to_hex(),
        signature: hex::encode(&signature[..]),
        deposit_data_root: hex::encode(deposit_data_root),
        withdrawal_credentials: hex::encode(withdrawal_credentials),
        fork_version,
    })
}
