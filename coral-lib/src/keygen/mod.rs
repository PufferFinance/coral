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
use ecies::PublicKey as EthPublicKey;

/// Generates a BLS keystore and returns the keygen payload.
/// This is the main entry point for BLS key generation.
pub fn generate_bls_keystore_handler(
    keygen_payload: AttestFreshBlsKeyPayload,
    keystore_password: &str,
) -> Result<BlsKeygenPayload> {
    generate_bls_keystore(
        keygen_payload.withdrawal_credentials,
        keygen_payload.guardian_pubkeys,
        keygen_payload.threshold,
        keystore_password,
        keygen_payload.fork_version,
    )
}

fn generate_bls_keystore(
    withdrawal_credentials: [u8; 32],
    guardian_public_keys: Vec<EthPublicKey>,
    threshold: usize,
    password: &str,
    fork_version: eth2_types::Version,
) -> Result<BlsKeygenPayload> {
    // Generate a SecretKeySet where t + 1 signature shares can be combined into a full signature.
    // attest_fresh_bls_key() function assumes `threshold = t + 1`, so we must pass new_bls_key(t=threshold - 1)
    let secret_key_set = bls_keys::new_bls_key(threshold - 1);

    // Shard the key into `n` keyshares
    let n = guardian_public_keys.len();
    let key_shares = bls_keys::distribute_key_shares(&secret_key_set, n);

    // Encrypt the shares using guardian pubkeys
    let mut encrypted_keys: Vec<EncryptedRecipientKeys> = Vec::new();
    for (g_pk, (sk_share, pk_share)) in guardian_public_keys.into_iter().zip(key_shares.into_iter())
    {
        let k = RecipientKeys {
            guardian_public_key: g_pk,
            secret_key_share: sk_share,
            public_key_share: pk_share,
        }
        .encrypt_to_recipient()?;
        encrypted_keys.push(k);
    }

    // Get validator aggregate public key
    let validator_pubkey = secret_key_set.public_keys().public_key();

    // Save validator private key to encrypted keystore
    bls_keys::save_bls_keystore(&secret_key_set, password)?;

    // Sign DepositMessage to deposit 32 ETH to beacon deposit contract
    let (signature, deposit_data_root) = eth2_signing::sign_full_deposit(
        &secret_key_set,
        withdrawal_credentials,
        fork_version,
    )?;

    // Return the payload - no enclave/remote attestation fields (always empty)
    Ok(BlsKeygenPayload {
        bls_pub_key_set: hex::encode(secret_key_set.public_keys().to_bytes()),
        bls_pub_key: validator_pubkey.to_hex(),
        signature: hex::encode(&signature[..]),
        deposit_data_root: hex::encode(deposit_data_root),
        bls_enc_priv_key_shares: encrypted_keys
            .iter()
            .map(|encrypted_key| encrypted_key.encrypted_secret_key_share_hex.clone())
            .collect(),
        intel_report: String::new(),
        intel_sig: String::new(),
        intel_x509: String::new(),
        guardian_eth_pub_keys: encrypted_keys
            .iter()
            .map(|k| crypto::eth_pk_to_hex_uncompressed(&k.guardian_public_key))
            .collect(),
        withdrawal_credentials: hex::encode(withdrawal_credentials),
        fork_version: eth2_types::GENESIS_FORK_VERSION,
    })
}

/// Keys to be encrypted for a recipient guardian
#[derive(Clone, Debug)]
pub struct RecipientKeys {
    pub guardian_public_key: EthPublicKey,
    pub secret_key_share: blsttc::SecretKeyShare,
    pub public_key_share: blsttc::PublicKeyShare,
}

/// Encrypted keys ready for transmission
#[derive(Clone, Debug)]
pub struct EncryptedRecipientKeys {
    pub guardian_public_key: EthPublicKey,
    pub public_key_share: blsttc::PublicKeyShare,
    pub encrypted_secret_key_share_hex: String,
}

impl RecipientKeys {
    /// ECIES envelope encrypt the BLS secret key share with the recipient ETH public key
    pub fn encrypt_to_recipient(&self) -> Result<EncryptedRecipientKeys> {
        let ct_sk = crypto::envelope_encrypt(
            &self.guardian_public_key,
            &self.secret_key_share.to_bytes(),
        )?;

        Ok(EncryptedRecipientKeys {
            guardian_public_key: self.guardian_public_key,
            public_key_share: self.public_key_share,
            encrypted_secret_key_share_hex: hex::encode(ct_sk),
        })
    }
}

