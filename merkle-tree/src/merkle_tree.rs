use crate::Hasher;
use std::marker::PhantomData;

pub struct MerkleTree<T: Hasher> {
    hash: PhantomData<T>,
    levels: Vec<Vec<T::Hash>>,
}

impl<T> MerkleTree<T>
where
    T: Hasher,
{
    pub fn build_level_helper(cur_leaves: &Vec<T::Hash>) -> Vec<Vec<T::Hash>> {
        if cur_leaves.len() == 1 {
            return vec![cur_leaves.clone()];
        }
        let mut upper_level: Vec<T::Hash> = vec![];
        let mut p = 0;
        while p < cur_leaves.len() {
            let (get_left_node, get_right_node) = (cur_leaves.get(p), cur_leaves.get(p + 1));
            let left_node = get_left_node.unwrap();
            let concatenated_hash = T::concat_and_hash(left_node, get_right_node);
            upper_level.push(concatenated_hash);
            p += 2;
        }
        return vec![
            vec![cur_leaves.clone()],
            Self::build_level_helper(&upper_level),
        ]
        .concat();
    }

    pub fn from_leaves(leaves: &Vec<T::Hash>) -> Self {
        let levels = Self::build_level_helper(leaves);
        Self {
            hash: PhantomData,
            levels,
        }
    }

    pub fn root_hex(self: &Self) -> Option<String> {
        if let Some(hash) = self.levels.last().unwrap().last() {
            return Some(T::hash_to_string(*hash));
        }
        return None;
    }

    pub fn find_neighbor(
        hash: T::Hash,
        level: &Vec<T::Hash>,
    ) -> (Option<T::Hash>, Option<T::Hash>) {
        for (leaf_node_index, leaf_node) in level.iter().enumerate() {
            if *leaf_node == hash {
                if leaf_node_index % 2 == 0 {
                    let get_right_node = level.get(leaf_node_index + 1);
                    return (
                        Some(T::concat_and_hash(leaf_node, get_right_node)),
                        get_right_node.copied(),
                    );
                } else {
                    let get_left_node = level.get(leaf_node_index - 1);
                    if let Some(left_node) = get_left_node {
                        return (
                            Some(T::concat_and_hash(left_node, Some(&leaf_node.clone()))),
                            Some(leaf_node.clone()),
                        );
                    }
                }
            }
        }
        (None, None)
    }

    pub fn find_path_to_root(self: &Self, hash: T::Hash) -> Vec<T::Hash> {
        let mut paths: Vec<T::Hash> = vec![];
        let mut current_hash = hash;
        for level in self.levels.iter() {
            let (get_concatenated_hash, get_neighbor_hash) =
                Self::find_neighbor(current_hash, &level);
            if level.len() == 1 {
                paths.push(get_concatenated_hash.unwrap());
            }
            if get_neighbor_hash.is_none() || get_concatenated_hash.is_none() {
                continue;
            }
            current_hash = get_concatenated_hash.unwrap();
            paths.push(get_neighbor_hash.unwrap());
        }
        return paths;
    }

    pub fn verify(self: &Self, hash: T::Hash) -> bool {
        let paths = self.find_path_to_root(hash);
        let cmp_root_hex = paths.last();
        match cmp_root_hex {
            Some(root_hex) => return self.root_hex().unwrap() == T::hash_to_string(*root_hex),
            None => false,
        }
    }

    pub fn print_tree(self: &Self) -> () {
        for (level_index, level) in self.levels.iter().enumerate() {
            for leaf_node in level {
                print!("{:?} - ", level_index);
                println!("{:?}", T::hash_to_string(*leaf_node));
            }
        }
    }

    pub fn depth(self: &Self) -> usize {
        return self.levels.len() - 1;
    }
}
