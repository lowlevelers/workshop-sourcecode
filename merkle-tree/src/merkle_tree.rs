use crate::Hasher;
use std::marker::PhantomData;

pub struct MerkleTree<T: Hasher> {
    hash: PhantomData<T>,
}

impl<T> MerkleTree<T>
where
    T: Hasher,
{
    pub fn root_hex() -> String {
        todo!("implement method to get the root hash in hexadecimal")
    }

    pub fn from_leaves() {
        todo!("implement method to build the tree from leaves")
    }

    pub fn verify() {
        todo!("implement method to verify the proof hash")
    }

    pub fn print_tree() {
        todo!("implement method to print the merkle tree levels")
    }
}
