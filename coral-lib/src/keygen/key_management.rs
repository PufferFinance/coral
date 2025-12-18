use anyhow::{Context, Result};
use std::fs;

use super::constants::BLS_KEYS_DIR;

/// Writes the BLS secret key to a keystore file
pub fn write_bls_keystore(pk_hex: &str, sk: &[u8], password: &str) -> Result<String> {
    // Create the keys dir if it does not exist
    fs::create_dir_all(BLS_KEYS_DIR).with_context(|| "Failed to create keys dir")?;

    // Sanitize inputs - strip 0x prefix if present
    let pk_hex = pk_hex.strip_prefix("0x").unwrap_or(pk_hex);
    let mut rng = rand::thread_rng();

    // Create encrypted keystore
    let uuid = eth_keystore::encrypt_key(BLS_KEYS_DIR, &mut rng, sk, password, Some(pk_hex))?;
    Ok(uuid)
}

