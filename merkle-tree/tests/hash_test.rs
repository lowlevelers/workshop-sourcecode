mod hash_test {
	use merkle_tree::{Hasher, Sha256Algorithm};

	#[test]
	fn test_sha256_hash_to_string() {
		let hash = Sha256Algorithm::hash("a".as_bytes());
		let end = Sha256Algorithm::hash_to_string(hash);
		let expected =
			String::from("ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb");
		assert_eq!(end, expected);
	}
}
