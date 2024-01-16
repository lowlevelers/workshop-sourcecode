use crate::hasher::Hasher;

#[derive(Clone)]
pub struct Sha256Algorithm {}

impl Hasher for Sha256Algorithm {
    type Hash = [u8; 64];

    fn hash(data: &[u8]) -> [u8; 64] {
        let data = sha256::digest(data);
        let mut slice: Self::Hash = [0; 64];
        for (d, s) in slice.iter_mut().zip(data.as_bytes().iter()) {
            *d = *s;
        }
        slice
    }

    fn hash_to_string(hash: Self::Hash) -> String {
        let mut result = String::default();
        for byte in hash.iter() {
            result.push(*byte as char);
        }
        result
    }
}
