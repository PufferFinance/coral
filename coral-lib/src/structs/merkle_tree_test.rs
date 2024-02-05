use super::merkle_tree::MerkleTree;

#[test]
fn test_no_leaf_nodes() {
    let tree = MerkleTree::from_leaf_nodes(vec![]);

    let merkle_root = tree.root_hash();
    assert_eq!([0u8; 32], merkle_root);
}
