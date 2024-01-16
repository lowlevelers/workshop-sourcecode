use core::convert::TryFrom;
use core::mem;

pub trait Hasher: Clone {
    type Hash: Copy + PartialEq + Into<Vec<u8>> + TryFrom<Vec<u8>>;

    fn hash(data: &[u8]) -> Self::Hash;

    fn concat_and_hash(left: &Self::Hash, right: Option<&Self::Hash>) -> Self::Hash {
        let mut concatenated: Vec<u8> = (*left).into();

        match right {
            Some(right_node) => {
                let mut right_node_clone: Vec<u8> = (*right_node).into();
                concatenated.append(&mut right_node_clone);
                Self::hash(&concatenated)
            }
            None => *left,
        }
    }

    fn hash_size() -> usize {
        mem::size_of::<Self::Hash>()
    }

    fn hash_to_string(hash: Self::Hash) -> String;
}
