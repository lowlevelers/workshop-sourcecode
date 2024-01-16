use merkle_tree::{Hasher, Sha256Algorithm};

pub struct TestData {
    pub leaf_values: Vec<String>,
    pub expected_root_hex: String,
    pub leaf_hashes: Vec<[u8; 64]>,
}

fn combine<T: Clone>(active: Vec<T>, rest: Vec<T>, mut combinations: Vec<Vec<T>>) -> Vec<Vec<T>> {
    return if rest.is_empty() {
        if active.is_empty() {
            combinations
        } else {
            combinations.push(active);
            combinations
        }
    } else {
        let mut next = active.clone();

        if let Some(first) = rest.first() {
            next.push(first.clone());
        }

        combinations = combine(next, rest.clone().drain(1..).collect(), combinations);
        combinations = combine(active, rest.clone().drain(1..).collect(), combinations);
        combinations
    };
}

/// Create all possible combinations of elements inside a vector without duplicates
pub fn combinations<T: Clone>(vec: Vec<T>) -> Vec<Vec<T>> {
    combine(Vec::new(), vec, Vec::new())
}

pub fn setup() -> TestData {
    let leaf_values = ["a", "b", "c", "d", "e", "f"];
    let expected_root_hex = "1f7379539707bcaea00564168d1d4d626b09b73f8a2a365234c62d763f854da2";
    let leaf_hashes: Vec<[u8; 64]> = leaf_values
        .iter()
        .map(|x| Sha256Algorithm::hash(x.as_bytes()))
        .collect();
    TestData {
        leaf_values: leaf_values.iter().cloned().map(String::from).collect(),
        leaf_hashes,
        expected_root_hex: String::from(expected_root_hex),
    }
}
