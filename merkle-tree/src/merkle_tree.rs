use crate::Hasher;
use std::marker::PhantomData;

pub struct MerkleTree<T: Hasher> {
    hash: PhantomData<T>,
}

impl<T> MerkleTree<T>
where
    T: Hasher,
{
    pub fn root_hex(self: &Self) -> Option<String> {
        todo!("implement method to get the root hash in hexadecimal")
    }

    pub fn from_leaves(_leaves: &Vec<T::Hash>) -> Self {
        todo!("implement method to build the tree from leaves")
    }

    pub fn verify(self: &Self) -> bool {
        todo!("implement method to verify the proof hash")
    }

    pub fn print_tree(self: &Self) -> () {
        todo!("implement method to print the merkle tree levels")
    }

    pub fn depth(self: &Self) -> usize {
        todo!("implement method to check the depth of the tree");
    }
}
