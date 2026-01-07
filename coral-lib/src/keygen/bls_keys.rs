use anyhow::{Context, Result};
use blsttc::{PublicKey, SecretKey};

use super::key_management::write_bls_keystore;

/// Generate a new BLS key pair
pub fn generate_bls_keypair() -> (SecretKey, PublicKey) {
    let sk = SecretKey::random();
    let pk = sk.public_key();
    (sk, pk)
}

/// Write the BLS secret key to an encrypted keystore using the hex encoded pk as filename
pub fn save_bls_keystore(sk: &SecretKey, password: &str) -> Result<String> {
    let pk_hex = sk.public_key().to_hex();
    let uuid = write_bls_keystore(&pk_hex, &sk.to_bytes(), password)
        .with_context(|| "BLS secret key failed to save")?;
    Ok(uuid)
}
