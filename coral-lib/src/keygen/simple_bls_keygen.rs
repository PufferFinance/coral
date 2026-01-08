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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_bls_keypair() {
        let (sk, pk) = generate_bls_keypair();

        // Verify that the public key matches the secret key
        assert_eq!(sk.public_key(), pk);

        // Verify key sizes
        assert_eq!(sk.to_bytes().len(), 32);
        assert_eq!(pk.to_bytes().len(), 48);
    }

    #[test]
    fn test_generate_bls_keypair_uniqueness() {
        let (sk1, pk1) = generate_bls_keypair();
        let (sk2, pk2) = generate_bls_keypair();

        // Each generated key pair should be unique
        assert_ne!(sk1.to_bytes(), sk2.to_bytes());
        assert_ne!(pk1.to_bytes(), pk2.to_bytes());
    }

    #[test]
    fn test_write_bls_keystore_to_dir() {
        let temp_dir = tempdir().unwrap();
        let (sk, pk) = generate_bls_keypair();
        let password = "testpassword123";

        let keystore_path =
            write_bls_keystore_to_dir(temp_dir.path(), &sk, &pk, password).unwrap();

        // Verify keystore file was created
        assert!(Path::new(&keystore_path).exists());

        // Verify filename contains public key
        let pk_hex = pk.to_hex();
        let pk_hex = pk_hex.strip_prefix("0x").unwrap_or(&pk_hex);
        assert!(keystore_path.contains(pk_hex));
        assert!(keystore_path.ends_with(".json"));
    }

    #[test]
    fn test_write_bls_keystore_decrypt() {
        let temp_dir = tempdir().unwrap();
        let (sk, pk) = generate_bls_keypair();
        let password = "testpassword123";

        let keystore_path =
            write_bls_keystore_to_dir(temp_dir.path(), &sk, &pk, password).unwrap();

        // Verify we can decrypt the keystore and recover the secret key
        let decrypted_sk = eth_keystore::decrypt_key(&keystore_path, password).unwrap();
        assert_eq!(decrypted_sk, sk.to_bytes());
    }

    #[test]
    fn test_generate_and_save_bls_key() {
        let temp_dir = tempdir().unwrap();
        let password = "testpassword123";

        let (pk_hex, keystore_path) =
            generate_and_save_bls_key(temp_dir.path(), password).unwrap();

        // Verify public key is valid hex (48 bytes = 96 hex chars)
        let pk_hex_stripped = pk_hex.strip_prefix("0x").unwrap_or(&pk_hex);
        assert_eq!(pk_hex_stripped.len(), 96);
        assert!(hex::decode(pk_hex_stripped).is_ok());

        // Verify keystore file exists
        assert!(Path::new(&keystore_path).exists());

        // Verify we can decrypt and the key is valid
        let decrypted_sk = eth_keystore::decrypt_key(&keystore_path, password).unwrap();
        let sk = SecretKey::from_bytes(decrypted_sk.try_into().unwrap()).unwrap();
        assert_eq!(sk.public_key().to_hex(), pk_hex);
    }

    #[test]
    fn test_generate_and_save_creates_directory() {
        let temp_dir = tempdir().unwrap();
        let nested_dir = temp_dir.path().join("nested").join("keys");
        let password = "testpassword123";

        // Directory doesn't exist yet
        assert!(!nested_dir.exists());

        let result = generate_and_save_bls_key(&nested_dir, password);
        assert!(result.is_ok());

        // Directory should now exist
        assert!(nested_dir.exists());
    }
}
