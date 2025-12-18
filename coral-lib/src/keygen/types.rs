use ecies::PublicKey as EthPublicKey;
use serde::{Deserialize, Serialize};

use super::eth2_types::Version;

/// Input payload for generating a fresh BLS key
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttestFreshBlsKeyPayload {
    #[serde(
        serialize_with = "serialize_pubkeys_hex",
        deserialize_with = "deserialize_pubkeys_from_hex"
    )]
    pub guardian_pubkeys: Vec<EthPublicKey>,
    #[serde(
        serialize_with = "serialize_as_32_bytes_array",
        deserialize_with = "deserialize_32_bytes_from_hex"
    )]
    pub withdrawal_credentials: [u8; 32],
    pub threshold: usize,
    pub fork_version: Version,
    /// Always false - we never do remote attestation
    #[serde(default)]
    pub do_remote_attestation: bool,
}

/// Output payload containing the generated BLS key information
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlsKeygenPayload {
    pub bls_pub_key_set: String,
    pub bls_pub_key: String,
    pub signature: String,
    pub deposit_data_root: String,
    pub bls_enc_priv_key_shares: Vec<String>,
    /// Always empty - no enclave support
    pub intel_sig: String,
    /// Always empty - no enclave support
    pub intel_report: String,
    /// Always empty - no enclave support
    pub intel_x509: String,
    pub guardian_eth_pub_keys: Vec<String>,
    pub withdrawal_credentials: String,
    pub fork_version: Version,
}

// Serialization helpers

fn serialize_pubkeys_hex<S>(pubkeys: &[EthPublicKey], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let hex_strings: Vec<String> = pubkeys
        .iter()
        .map(|pubkey| hex::encode(pubkey.serialize()))
        .collect();

    let mut seq = serializer.serialize_seq(Some(hex_strings.len()))?;
    for hex_string in hex_strings {
        seq.serialize_element(&hex_string)?;
    }
    seq.end()
}

fn deserialize_pubkeys_from_hex<'de, D>(deserializer: D) -> Result<Vec<EthPublicKey>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct HexVisitor;

    impl<'de> serde::de::Visitor<'de> for HexVisitor {
        type Value = Vec<EthPublicKey>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of hex strings")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut pubkeys = Vec::new();
            while let Some(hex_string) = seq.next_element::<String>()? {
                let hex_string = hex_string.strip_prefix("0x").unwrap_or(&hex_string);
                let bytes = hex::decode(hex_string).map_err(serde::de::Error::custom)?;
                let pubkey =
                    EthPublicKey::parse_slice(&bytes, None).map_err(serde::de::Error::custom)?;
                pubkeys.push(pubkey);
            }
            Ok(pubkeys)
        }
    }

    deserializer.deserialize_seq(HexVisitor)
}

fn serialize_as_32_bytes_array<S>(data: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex_str = hex::encode(data);
    serializer.serialize_str(&hex_str)
}

fn deserialize_32_bytes_from_hex<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct HexVisitor;

    impl<'de> serde::de::Visitor<'de> for HexVisitor {
        type Value = [u8; 32];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string representing a 32-byte array in hexadecimal format")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let value = value.strip_prefix("0x").unwrap_or(value);
            let bytes = hex::decode(value).map_err(E::custom)?;
            if bytes.len() != 32 {
                return Err(E::custom(format!("Expected 32 bytes, got {}", bytes.len())));
            }
            let mut array = [0; 32];
            array.copy_from_slice(&bytes);
            Ok(array)
        }
    }

    deserializer.deserialize_str(HexVisitor)
}

