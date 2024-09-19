use colored::Colorize;
use ethers::types::U256;
use ethers::utils::hex;
use std::fs::File;
use std::io::Read;

use coral_lib::error::{AppError, AppErrorKind, AppResult};
use coral_lib::structs::merkle_tree::{verify_merkle_proof, MerkleTree};
use coral_lib::structs::rewards_file::RewardsRawFile;
use coral_lib::structs::rewards_tree::{
    generate_merkle_leaf, NoOpReward, RewardValidatorMerkleData,
};
use coral_lib::utils::ethereum::{get_provider, is_contract_address};
use coral_lib::utils::parse::parse_address;

/// Verify the merkle tree rewards data from a given rewards file
pub async fn verify_merkle_tree_rewards(rewards_file: String, rpc_url: String) -> AppResult {
    let provider = get_provider(&rpc_url)?;

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

    // Aggregate total rewards per node operator and generate merkle leaves
    let mut validator_hashes: Vec<RewardValidatorMerkleData> = Vec::new();
    for (add, operator) in &rewards.node_operators {
        let address = parse_address(add.to_string()).map_err(|_| {
            AppError::new(
                AppErrorKind::ParseError,
                format!("Invalid address format: {}", add),
            )
        })?;
        let total_rewards_wei = operator.total * U256::exp10(9);
        let is_contract = is_contract_address(&provider, address).await?;

        let leaf = generate_merkle_leaf(address, is_contract, total_rewards_wei);

        validator_hashes.push(RewardValidatorMerkleData {
            hash: leaf,
            node: address,
            reward: NoOpReward {
                address,
                total_rewards: total_rewards_wei,
            },
            is_contract,
        });
    }

    // Sort validator_hashes based on the hash
    validator_hashes.sort_by(|a, b| a.hash.cmp(&b.hash));

    // Extract sorted leaves
    let leaves: Vec<[u8; 32]> = validator_hashes.iter().map(|data| data.hash).collect();

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
        let proof_hex = hex::encode(
            proof
                .iter()
                .flat_map(|x| x.iter())
                .copied()
                .collect::<Vec<u8>>(),
        );
        println!(
            "{}",
            format!("Proof for index {}: {}", index, proof_hex).yellow()
        );

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

    // Test case for verifying the merkle tree rewards data
    // from a given rewards file
    // Data can be found in the calldata of
    // https://holesky.etherscan.io/tx/0xc26cb92fdde6735cbb0e1d00bb1d6bc35cd228c2762dbcd6d3a19d87e934046b
    #[test]
    fn test_verify_merkle_tree_rewards_ok() {
        let rt = Runtime::new().unwrap();

        let holesky_rpc_url = "https://ethereum-holesky-rpc.publicnode.com";

        let mut test_file_path = get_base_path();
        test_file_path.push("src/tests/rewards-files/withdrawal_reward_52235.json");

        rt.block_on(async {
            let result = verify_merkle_tree_rewards(
                test_file_path.to_string_lossy().to_string(),
                holesky_rpc_url.to_string(),
            )
            .await;
            println!("verify_merkle_tree_rewards result {:?}", result);
            assert!(result.is_ok(), "The verification should succeed.");
        });
    }

    // Second test case for verifying the merkle tree rewards data
    // from a given rewards file
    // Data can be found in the calldata of
    // https://holesky.etherscan.io/tx/0x3b002866c89e57c21935d0b6192ad423581ddfb63c8fb709dfc3b6d1a43957b4
    #[test]
    fn test_verify_merkle_tree_rewards_ok_2() {
        let rt = Runtime::new().unwrap();

        let holesky_rpc_url = "https://ethereum-holesky-rpc.publicnode.com";

        let mut test_file_path = get_base_path();
        test_file_path.push("src/tests/rewards-files/withdrawal_reward_65286.json");

        rt.block_on(async {
            let result = verify_merkle_tree_rewards(
                test_file_path.to_string_lossy().to_string(),
                holesky_rpc_url.to_string(),
            )
            .await;
            println!("verify_merkle_tree_rewards result {:?}", result);
            assert!(result.is_ok(), "The verification should succeed.");
        });
    }
}
