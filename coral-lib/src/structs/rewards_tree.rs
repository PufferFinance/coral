use ethers::abi::Token;
use ethers::prelude::*;
use ethers::types::{Address, U256};
use ethers::utils::keccak256;
use serde::{Deserialize, Serialize};

/// Reward data for a single node operator
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoOpReward {
    /// Node operator address from which we calculate the total rewards
    pub address: Address,
    /// Total of the rewards for all the validators
    pub total_rewards: U256,
}

/// Reward merkle tree data with hash, node address and rewards list l
#[derive(Clone, Debug)]
pub struct RewardValidatorMerkleData {
    pub hash: [u8; 32],
    pub node: Address,
    pub reward: NoOpReward,
}

/// Helper to generate merkle tree leaf
/// With this leaf model
/// keccak256(abi.encode(noopAddress, startEpoch, endEpoch, total))
pub fn generate_merkle_leaf(
    address: Address,
    start_epoch: u64,
    end_epoch: u64,
    total: U256,
) -> [u8; 32] {
    let calldata = abi::encode(&[
        Token::Address(address),
        Token::Uint(U256::from(start_epoch)),
        Token::Uint(U256::from(end_epoch)),
        Token::Uint(total),
    ]);
    keccak256(calldata)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::merkle_tree::{verify_merkle_proof, MerkleTree};
    use ethers::types::U256;
    use ethers::utils::hex;

    #[test]
    fn test_merkle_tree_with_rewards() {
        // Setup rewards data
        let reward1 = NoOpReward {
            address: "0xbdadfc936fa42bcc54f39667b1868035290a0241"
                .parse()
                .unwrap(),
            total_rewards: U256::from(5000000),
        };
        let reward2 = NoOpReward {
            address: "0xdddeafb492752fc64220ddb3e7c9f1d5cccdfdf0"
                .parse()
                .unwrap(),
            total_rewards: U256::from(15000000),
        };

        // Generate merkle leaves
        let leaf1 = generate_merkle_leaf(reward1.address, 61057, 61179, reward1.total_rewards);
        let leaf2 = generate_merkle_leaf(reward2.address, 61057, 61179, reward2.total_rewards);

        let leaf_nodes = vec![leaf1, leaf2];
        let merkle_tree = MerkleTree::from_leaf_nodes(leaf_nodes.clone());

        // Build merkle root and proofs
        let root_hash = merkle_tree.root_hash();
        println!("Merkle root: {:?}", hex::encode(root_hash));

        let proof1 = merkle_tree.generate_proof(0);
        let proof2 = merkle_tree.generate_proof(1);
        println!("Merkle proof for noop first reward: {:?}", proof1);
        println!("Merkle proof for noop second reward: {:?}", proof2);

        // Verify proof for every leaf
        assert!(verify_merkle_proof(root_hash, leaf1, &proof1));
        assert!(verify_merkle_proof(root_hash, leaf2, &proof2));
    }
}
