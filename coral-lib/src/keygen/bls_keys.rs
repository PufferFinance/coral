use anyhow::{Context, Result};
use blsttc::{PublicKeyShare, SecretKeySet, SecretKeyShare};

use super::key_management::write_bls_keystore;

/// Generate a new BLS secret key set with the given threshold
pub fn new_bls_key(threshold: usize) -> SecretKeySet {
    let mut rng = rand::thread_rng();
    let sk_set = SecretKeySet::random(threshold, &mut rng);
    assert!(sk_set.threshold() == threshold);
    sk_set
}

/// Write the BLS secret key to an encrypted keystore using the hex encoded pk as filename
pub fn save_bls_keystore(sk_set: &SecretKeySet, password: &str) -> Result<String> {
    // Hex-encode pk
    let pk_hex = sk_set.public_keys().public_key().to_hex();

    // Save keystore
    let uuid = write_bls_keystore(&pk_hex, &sk_set.secret_key().to_bytes(), password)
        .with_context(|| "aggregate bls sk failed to save")?;
    Ok(uuid)
}

/// Distributes `n` key shares from a given BLS `SecretKeySet`.
/// Returns a vector of tuples containing the `SecretKeyShare` and corresponding `PublicKeyShare` for each node.
pub fn distribute_key_shares(
    sk_set: &SecretKeySet,
    n: usize,
) -> Vec<(SecretKeyShare, PublicKeyShare)> {
    let pk_set = sk_set.public_keys();

    (0..n)
        .map(|id| {
            let sk_share = sk_set.secret_key_share(id);
            let pk_share = pk_set.public_key_share(id);
            (sk_share, pk_share)
        })
        .collect()
}

