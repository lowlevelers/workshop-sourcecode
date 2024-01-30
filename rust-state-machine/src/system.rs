use num::{One, Zero};
use std::{collections::BTreeMap, hash::Hash, ops::AddAssign};

pub trait Config {
	type Address: Hash + Clone + Ord;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + AddAssign + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::Address, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		self.block_number.add_assign(T::BlockNumber::one());
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::Address) {
		let nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	use super::Pallet;

	#[test]
	fn init_system() {
		struct TestConfig;
		impl super::Config for TestConfig {
			type Address = String;
			type BlockNumber = u32;
			type Nonce = u32;
		}

		let mut system = Pallet::<TestConfig>::new();
		let alice = String::from("alice");

		system.inc_block_number();
		system.inc_nonce(&alice);

		assert_eq!(system.block_number(), 1);
		let expected_nonce = *system.nonce.get(&alice).unwrap_or(&0);
		assert_eq!(expected_nonce, 1);
	}
}
