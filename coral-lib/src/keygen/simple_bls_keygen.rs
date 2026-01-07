use anyhow::{Context, Result};
use blsttc::{PublicKey, SecretKey};
use std::fs;
use std::path::Path;

/// Generate a new BLS key pair
pub fn generate_bls_keypair() -> (SecretKey, PublicKey) {
    let sk = SecretKey::random();
    let pk = sk.public_key();
    (sk, pk)
}

/// Write BLS keystore to a directory with pubkey as filename
/// Returns the path to the keystore file
pub fn write_bls_keystore_to_dir(
    output_dir: &Path,
    sk: &SecretKey,
    pk: &PublicKey,
    password: &str,
) -> Result<String> {
    // Get hex-encoded public key (strip 0x prefix if present)
    let pk_hex = pk.to_hex();
    let pk_hex = pk_hex.strip_prefix("0x").unwrap_or(&pk_hex);

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

    // Create encrypted keystore with pubkey.json as filename
    let filename = format!("{}.json", pk_hex);
    let mut rng = rand::thread_rng();
    let _uuid = eth_keystore::encrypt_key(output_dir, &mut rng, &sk.to_bytes(), password, Some(&filename))
        .with_context(|| "Failed to encrypt and save keystore")?;

    let keystore_path = output_dir.join(&filename);
    Ok(keystore_path.to_string_lossy().to_string())
}

/// Generate a BLS key pair and save it to a keystore
/// Returns (public_key_hex, keystore_path)
pub fn generate_and_save_bls_key(output_dir: &Path, password: &str) -> Result<(String, String)> {
    let (sk, pk) = generate_bls_keypair();
    let pk_hex = pk.to_hex();
    let keystore_path = write_bls_keystore_to_dir(output_dir, &sk, &pk, password)?;
    Ok((pk_hex, keystore_path))
}
