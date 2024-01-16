use crate::Hasher;
use std::marker::PhantomData;

pub struct MerkleTree<T: Hasher> {
    hash: PhantomData<T>,
}

impl<T> MerkleTree<T> where T: Hasher {}
