use colored::Colorize;
use ethers::types::{Address, U256};
use ethers::utils::hex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use coral_lib::error::{AppError, AppErrorKind, AppResult};
use coral_lib::structs::merkle_tree::{verify_merkle_proof, MerkleTree};
use coral_lib::structs::rewards_file::RewardsRawFile;
use coral_lib::structs::rewards_tree::generate_merkle_leaf;
use coral_lib::utils::parse::parse_address;

/// Verify the merkle tree rewards data from a given rewards file
pub async fn verify_merkle_tree_rewards(rewards_file: String) -> AppResult {
    // open and read rewards file
    let mut file = File::open(&rewards_file).map_err(|err| {
        AppError::new(
            AppErrorKind::OpenFileError,
            format!("Failed to open rewards file: {}", err),
        )
    })?;

    let mut data = String::new();
    file.read_to_string(&mut data).map_err(|err| {
        AppError::new(
            AppErrorKind::ReadFileError,
            format!("Failed to read rewards file: {}", err),
        )
    })?;

    // Deserialize the rewards file
    let rewards: RewardsRawFile = serde_json::from_str(&data).map_err(|err| {
        AppError::new(
            AppErrorKind::ParseError,
            format!("Failed to parse rewards file: {}", err),
        )
    })?;

    // Aggregate total rewards per node operator
    let mut noops_list: HashMap<Address, U256> = HashMap::new();
    for (add, operator) in &rewards.node_operators {
        let address = parse_address(add.to_string()).map_err(|_| {
            AppError::new(
                AppErrorKind::ParseError,
                format!("Invalid address format: {}", add),
            )
        })?;
        let total_rewards_for_operator = U256::from_dec_str(&operator.total).map_err(|_| {
            AppError::new(
                AppErrorKind::ParseError,
                format!("Failed to parse U256 from string: {}", operator.total),
            )
        })?;

        let entry = noops_list.entry(address).or_insert_with(U256::zero);
        *entry += total_rewards_for_operator;
    }

    // Generate merkle leaves
    let mut leaves = Vec::new();
    for (&address, &total_reward) in &noops_list {
        let leaf = generate_merkle_leaf(
            address,
            rewards.metadata.start_epoch,
            rewards.metadata.end_epoch,
            total_reward,
        );
        leaves.push(leaf);
    }

    // Generate the Merkle tree and root hash
    let merkle_tree = MerkleTree::from_leaf_nodes(leaves.clone());
    let computed_root_hash = merkle_tree.root_hash();
    let computed_root_hex = hex::encode(computed_root_hash);

    println!(
        "{}",
        format!("Computed Merkle root: {}", computed_root_hex).blue()
    );

    // Compare with merkle root in the file
    if computed_root_hex.to_lowercase() != rewards.merkle_root.to_lowercase() {
        return Err(AppError::new(
            AppErrorKind::MerkleTreeRootInvalid,
            format!(
                "Merkle root mismatch: expected {}, found {}",
                rewards.merkle_root, computed_root_hex
            ),
        ));
    }

    // Verify now each leaf against the merkle root using its proof
    for (index, leaf) in leaves.iter().enumerate() {
        let proof = merkle_tree.generate_proof(index);
        if !verify_merkle_proof(computed_root_hash, *leaf, &proof) {
            return Err(AppError::new(
                AppErrorKind::MerkleProofInvalid,
                format!("Merkle proof verification failed for leaf index {}", index),
            ));
        }
    }

    println!(
        "{}",
        "All merkle proofs verified successfully for the given data."
            .to_string()
            .green()
    );
    println!(
        "{}",
        format!("Merkle root matches the provided hash in rewards file {rewards_file}.").green()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use tokio::runtime::Runtime;

    fn get_base_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn test_verify_merkle_tree_rewards() {
        let rt = Runtime::new().unwrap();

        let mut test_file_path = get_base_path();
        test_file_path.push("src/tests/rewards-files/rewards-test-1.json");

        rt.block_on(async {
            let result =
                verify_merkle_tree_rewards(test_file_path.to_string_lossy().to_string()).await;
            println!("verify_merkle_tree_rewards result {:?}", result);
            assert!(result.is_ok(), "The verification should succeed.");
        });
    }
}
