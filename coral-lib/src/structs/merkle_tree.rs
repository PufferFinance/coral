use ethers::abi::AbiEncode;
use ethers::utils::keccak256;

use std::slice::Iter;

#[derive(Clone, Debug)]
pub struct MerkleTree {
    pub layers: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    pub fn from_leaf_nodes(mut leaf_nodes: Vec<[u8; 32]>) -> Self {
        let leaf_nodes_count = leaf_nodes.len();

        let mut full_tree_node_count = 1;
        // find a power of 2 such that all
        // leaf nodes can fit into this tree
        loop {
            // pad empty leaf nodes with 0x0
            if full_tree_node_count >= leaf_nodes_count {
                for _ in 0..(full_tree_node_count - leaf_nodes_count) {
                    leaf_nodes.push([0; 32]);
                }
                break;
            }
            full_tree_node_count *= 2;
        }

        let mut node_stack = leaf_nodes;

        let mut layers = vec![];

        // consolidate until we only have 1 leaf node left
        while node_stack.len() != 1 {
            let mut new_node_stack = vec![];
            for i in (0..node_stack.len()).step_by(2) {
                let mut left = node_stack[i];
                let mut right = node_stack[i + 1];

                if right <= left {
                    std::mem::swap(&mut right, &mut left);
                }
                let combined: Vec<u8> = left.iter().chain(right.iter()).copied().collect();
                new_node_stack.push(keccak256(combined))
            }
            layers.push(node_stack);
            node_stack = new_node_stack;
        }
        layers.push(node_stack);

        Self { layers }
    }

    pub fn leaf_nodes(&self) -> Iter<[u8; 32]> {
        self.layers[0].iter()
    }

    pub fn root_hash(&self) -> [u8; 32] {
        let layer_count = self.layers.len();
        self.layers[layer_count - 1][0]
    }

    pub fn generate_proof(&self, leaf_index: usize) -> Vec<[u8; 32]> {
        let mut merkle_proof = Vec::new();

        let mut curr_index = leaf_index;
        for layer in &self.layers[..self.layers.len() - 1] {
            let neighbor_index = if curr_index % 2 == 0 {
                curr_index + 1
            } else {
                curr_index - 1
            };
            merkle_proof.push(layer[neighbor_index]);
            curr_index /= 2;
        }
        merkle_proof
    }

    pub fn print_layers(&self) {
        for layer in self.layers.iter() {
            for node in layer.iter() {
                println!("{}", node.encode_hex());
            }
            println!();
        }
    }
}

pub fn verify_merkle_proof(root: [u8; 32], leaf: [u8; 32], proofs: &[[u8; 32]]) -> bool {
    let computed_root = proofs.iter().fold(leaf, |acc, x| {
        if acc < *x {
            let left = acc;
            let right = x;
            let combined: Vec<_> = left.iter().chain(right.iter()).copied().collect();
            keccak256(combined)
        } else {
            let left = x;
            let right = acc;
            let combined: Vec<_> = left.iter().chain(right.iter()).copied().collect();
            keccak256(combined)
        }
    });

    root == computed_root
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::rewards_tree::generate_merkle_leaf;
    use ethers::types::{Address, U256};
    use ethers::utils::hex;
    #[test]
    fn test_merkle_tree_and_verify_proof() {
        let address1: Address = "0xbdadfc936fa42bcc54f39667b1868035290a0241"
            .parse()
            .unwrap();
        let address2: Address = "0xdddeafb492752fc64220ddb3e7c9f1d5cccdfdf0"
            .parse()
            .unwrap();

        let leaf1 = generate_merkle_leaf(address1, 61180, 61190, U256::from(6000));
        let leaf2 = generate_merkle_leaf(address2, 61180, 61190, U256::from(4000));

        let leaf_nodes = vec![leaf1, leaf2];
        let merkle_tree = MerkleTree::from_leaf_nodes(leaf_nodes.clone());

        let root_hash = merkle_tree.root_hash();
        println!("Merkle Root: {:?}", hex::encode(root_hash));

        let proof: Vec<[u8; 32]> = merkle_tree.generate_proof(0);
        println!("Merkle Proof: {:?}", proof);
        assert!(verify_merkle_proof(root_hash, leaf1, &proof));
    }
}
