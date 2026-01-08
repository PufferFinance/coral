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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;

    fn setup_test_keys_dir() {
        // Ensure the keys directory exists
        let _ = fs::create_dir_all("./etc/keys/bls_keys");
    }

    fn cleanup_test_keys() {
        // Clean up any keys created during tests
        let _ = fs::remove_dir_all("./etc/keys/bls_keys");
    }

    #[test]
    #[serial]
    fn test_generate_bls_keystore_handler() {
        setup_test_keys_dir();

        let payload = AttestFreshBlsKeyPayload {
            withdrawal_credentials: [0x01u8; 32],
            fork_version: [0x00, 0x00, 0x00, 0x00],
        };
        let password = "testpassword123";

        let result = generate_bls_keystore_handler(payload.clone(), password);
        assert!(result.is_ok(), "Failed: {:?}", result.err());

        let keygen_payload = result.unwrap();

        // Verify public key format (48 bytes = 96 hex chars)
        let pk_hex = keygen_payload
            .bls_pub_key
            .strip_prefix("0x")
            .unwrap_or(&keygen_payload.bls_pub_key);
        assert_eq!(pk_hex.len(), 96);
        assert!(hex::decode(pk_hex).is_ok());

        // Verify signature format (96 bytes = 192 hex chars)
        assert_eq!(keygen_payload.signature.len(), 192);
        assert!(hex::decode(&keygen_payload.signature).is_ok());

        // Verify deposit data root format (32 bytes = 64 hex chars)
        assert_eq!(keygen_payload.deposit_data_root.len(), 64);
        assert!(hex::decode(&keygen_payload.deposit_data_root).is_ok());

        // Verify withdrawal credentials are correctly encoded
        assert_eq!(keygen_payload.withdrawal_credentials, hex::encode([0x01u8; 32]));

        // Verify fork version is preserved
        assert_eq!(keygen_payload.fork_version, payload.fork_version);

        cleanup_test_keys();
    }

    #[test]
    #[serial]
    fn test_generate_bls_keystore_handler_mainnet_fork() {
        setup_test_keys_dir();

        let payload = AttestFreshBlsKeyPayload {
            withdrawal_credentials: [0x01u8; 32],
            fork_version: [0x00, 0x00, 0x00, 0x01], // Mainnet genesis fork version
        };
        let password = "securepassword";

        let result = generate_bls_keystore_handler(payload.clone(), password);
        assert!(result.is_ok(), "Failed: {:?}", result.err());

        let keygen_payload = result.unwrap();
        assert_eq!(keygen_payload.fork_version, [0x00, 0x00, 0x00, 0x01]);

        cleanup_test_keys();
    }

    #[test]
    #[serial]
    fn test_generate_bls_keystore_handler_creates_keystore_file() {
        setup_test_keys_dir();

        let payload = AttestFreshBlsKeyPayload {
            withdrawal_credentials: [0x02u8; 32],
            fork_version: [0x00, 0x00, 0x00, 0x00],
        };
        let password = "testpassword123";

        let keygen_payload = generate_bls_keystore_handler(payload, password).unwrap();

        // Check that keystore file exists
        let pk_hex = keygen_payload
            .bls_pub_key
            .strip_prefix("0x")
            .unwrap_or(&keygen_payload.bls_pub_key);
        let keystore_path = format!("./etc/keys/bls_keys/{}", pk_hex);
        assert!(
            std::path::Path::new(&keystore_path).exists(),
            "Keystore file should exist at {}",
            keystore_path
        );

        cleanup_test_keys();
    }

    #[test]
    #[serial]
    fn test_generate_bls_keystore_handler_unique_keys() {
        setup_test_keys_dir();

        let payload = AttestFreshBlsKeyPayload {
            withdrawal_credentials: [0x01u8; 32],
            fork_version: [0x00, 0x00, 0x00, 0x00],
        };
        let password = "testpassword123";

        let result1 = generate_bls_keystore_handler(payload.clone(), password).unwrap();
        let result2 = generate_bls_keystore_handler(payload, password).unwrap();

        // Each call should generate a unique key
        assert_ne!(result1.bls_pub_key, result2.bls_pub_key);
        assert_ne!(result1.signature, result2.signature);
        assert_ne!(result1.deposit_data_root, result2.deposit_data_root);

        cleanup_test_keys();
    }

    #[test]
    #[serial]
    fn test_generate_bls_keystore_eth1_withdrawal_credentials() {
        setup_test_keys_dir();

        // ETH1 withdrawal credentials start with 0x01
        let mut wc = [0u8; 32];
        wc[0] = 0x01;
        // Rest would typically be the ETH1 address hash

        let payload = AttestFreshBlsKeyPayload {
            withdrawal_credentials: wc,
            fork_version: [0x00, 0x00, 0x00, 0x00],
        };
        let password = "testpassword123";

        let result = generate_bls_keystore_handler(payload, password);
        assert!(result.is_ok(), "Failed: {:?}", result.err());

        let keygen_payload = result.unwrap();
        let decoded_wc = hex::decode(&keygen_payload.withdrawal_credentials).unwrap();
        assert_eq!(decoded_wc[0], 0x01);

        cleanup_test_keys();
    }
}
