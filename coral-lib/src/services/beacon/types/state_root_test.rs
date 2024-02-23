use ethers::types::{H256, U256};
use serde_json::json;

use super::StateId;

#[test]
pub fn test_serialize_state_id() {
    let values = [
        (StateId::Head, "head"),
        (StateId::Genesis, "genesis"),
        (StateId::Finalized, "finalized"),
        (StateId::Justified, "justified"),
        (StateId::Slot(U256::from(1000)), "1000"),
        (
            StateId::StateRoot(H256::from_low_u64_be(123)),
            "0x000000000000000000000000000000000000000000000000000000000000007b",
        ),
    ];

    for (value, value_str) in values {
        let commit = json!({
            "value": value,
        });

        let json = serde_json::to_string(&commit).unwrap();
        assert_eq!(json, format!("{{\"value\":\"{value_str}\"}}"))
    }
}

#[test]
fn test_deserialize_state_id() {
    let values = [
        (StateId::Head, "head"),
        (StateId::Genesis, "genesis"),
        (StateId::Finalized, "finalized"),
        (StateId::Justified, "justified"),
        (StateId::Slot(U256::from(1000)), "1000"),
        (
            StateId::StateRoot(H256::from_low_u64_be(123)),
            "0x000000000000000000000000000000000000000000000000000000000000007b",
        ),
    ];

    for (value, value_str) in values {
        let commit = json!({
            "value": value,
        });
        let commit_str = format!("{{\"value\":\"{value_str}\"}}");

        let json: serde_json::Value = serde_json::from_str(&commit_str).unwrap();
        assert_eq!(commit, json);
    }
}

#[test]
pub fn test_display_state_id() {
    let values = [
        (StateId::Head, "head"),
        (StateId::Genesis, "genesis"),
        (StateId::Finalized, "finalized"),
        (StateId::Justified, "justified"),
        (
            StateId::Slot(U256::from(1000).saturating_mul(U256::from(10000000000000000000u64))),
            "10000000000000000000000",
        ),
        (
            StateId::StateRoot(H256::from_low_u64_be(123)),
            "0x000000000000000000000000000000000000000000000000000000000000007b",
        ),
    ];

    for (value, value_str) in values {
        let result_str = format!("{}", value);
        assert_eq!(result_str, value_str)
    }
}
