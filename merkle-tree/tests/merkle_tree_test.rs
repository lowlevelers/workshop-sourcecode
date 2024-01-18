mod common;

pub mod root {
    use crate::common;
    use merkle_tree::{MerkleTree, Sha256Algorithm};

    #[test]
    pub fn should_return_a_correct_root() {
        let test_cases = common::get_test_cases();
        for i in 0..test_cases.len() {
            println!("---------------");
            let test_data = common::setup(i);
            let merkle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&test_data.leaf_hashes);
            merkle_tree.print_tree();
            assert_eq!(
                merkle_tree.root_hex(),
                Some(test_data.expected_root_hex.to_string())
            );
        }
    }
}

pub mod tree_depth {
    use crate::common;
    use merkle_tree::{Hasher, MerkleTree, Sha256Algorithm};

    #[test]
    pub fn should_return_a_correct_tree_depth() {
        let test_data = common::setup(0);

        let merkle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&test_data.leaf_hashes);

        merkle_tree.print_tree();

        let paths = merkle_tree.find_path_to_root(test_data.leaf_hashes[0]);
        for leaf_node in paths {
            println!("{:?}", Sha256Algorithm::hash_to_string(leaf_node));
        }

        let depth = merkle_tree.depth();
        assert_eq!(depth, 3)
    }
}
