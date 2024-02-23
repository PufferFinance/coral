use ethers::types::U256;
use serde::{de, Deserialize, Deserializer};

pub fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    s.parse::<u64>()
        .map_err(|_| de::Error::custom("failed to parse u64".to_string()))
}

pub fn string_to_u256_base10<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    U256::from_str_radix(s, 10).map_err(|_| de::Error::custom("failed to parse U256".to_string()))
}

pub fn u256_serialize<S>(val: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&val.to_string())
}
