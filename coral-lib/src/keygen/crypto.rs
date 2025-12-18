use anyhow::Result;
use ecies::PublicKey as EthPublicKey;

/// Converts SECP256K1 key to uncompressed 65 bytes then hex-encodes
pub fn eth_pk_to_hex_uncompressed(pk: &EthPublicKey) -> String {
    hex::encode(pk.serialize())
}

/// Use ECIES to encrypt the message using the provided public key. The encrypted message
/// can only be decrypted by the owner of the corresponding private key.
pub fn envelope_encrypt(public_key: &EthPublicKey, message: &[u8]) -> Result<Vec<u8>> {
    let encrypted_message = ecies::encrypt(&public_key.serialize(), message).map_err(|e| {
        anyhow::anyhow!(
            "Failed to encrypt the message using the provided public key: {:?}",
            e
        )
    })?;

    Ok(encrypted_message)
}

