use merkle_tree::{Hasher, Sha256Algorithm};

pub struct TestData {
	pub leaf_values: Vec<String>,
	pub expected_root_hex: String,
	pub leaf_hashes: Vec<[u8; 64]>,
}

pub fn get_test_cases() -> Vec<(Vec<&'static str>, String)> {
	let test_cases: Vec<(Vec<&str>, String)> = vec![
		(
			vec!["A", "B", "C", "D", "E", "F", "G", "H"],
			String::from("e3caa45a951457b84493e3adec8265f99311c3b1b4f28befbb067a1912efab9e"),
		),
		(
			vec!["A", "B", "C", "D", "E"],
			String::from("fcf69d8aabe9846da1a7e70b74595952cc817738fe9618d4b8e0d2bc8bc3b980"),
		),
		(
			vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N"],
			String::from("0d7ad7e793e6fd74203a8e82d97b38750a9b5a0655d3b72f1d64998a3ea7ea67"),
		),
		(
			vec!["Alice", "Bob", "Allen", "Thomas", "Maverick", "Stuart"],
			String::from("12dc8922811583d3fc203df5afacaf44390afed0773b924d6c0343bcdeb21088"),
		),
		(
			vec![
				"Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
				"Pluto",
			],
			String::from("92788393914b00cc717c04a432b25793c184dc5202c45acdeee698eed7b62e3e"),
		),
	];
	return test_cases;
}

pub fn setup(index: usize) -> TestData {
	let (leaf_values, expected_root_hex) = get_test_cases()[index].clone();
	let leaf_hashes: Vec<[u8; 64]> =
		leaf_values.iter().map(|x| Sha256Algorithm::hash(x.as_bytes())).collect();
	TestData {
		leaf_values: leaf_values.iter().cloned().map(String::from).collect(),
		leaf_hashes,
		expected_root_hex: String::from(expected_root_hex),
	}
}
