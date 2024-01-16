mod common;

pub mod root {
    use crate::common;
    use merkle_tree::{MerkleTree, Sha256Algorithm};

    #[test]
    pub fn should_return_a_correct_root() {
        let test_data = common::setup();
        let merkle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&test_data.leaf_hashes);
        assert_eq!(
            merkle_tree.root_hex(),
            Some(test_data.expected_root_hex.to_string())
        );
    }
}

pub mod tree_depth {
    use crate::common;
    use merkle_tree::{MerkleTree, Sha256Algorithm};

    #[test]
    pub fn should_return_a_correct_tree_depth() {
        let test_data = common::setup();

        let merkle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&test_data.leaf_hashes);

        let depth = merkle_tree.depth();
        assert_eq!(depth, 3)
    }
}
