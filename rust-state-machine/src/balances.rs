use num::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

use crate::system;

pub trait Config: system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::Address, T::Balance>,
}

impl<T> Pallet<T>
where
	T: Config,
{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &T::Address, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &T::Address) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: T::Address,
		to: T::Address,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		if caller == to {
			return Err("transfer to same address");
		}

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::{system, types::Balance};

	struct TestConfig;
	impl system::Config for TestConfig {
		type Address = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl super::Config for TestConfig {
		type Balance = Balance;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();
		let (alice, bob) = (String::from("alice"), String::from("bob"));
		let expected = balances.transfer(alice.clone(), bob.clone(), 20).is_err();
		assert!(expected);

		balances.set_balance(&alice, 100);
		balances.set_balance(&bob, 200);
		balances.transfer(alice.clone(), bob.clone(), 20).unwrap();

		assert_eq!(balances.balance(&alice), 80);
		assert_eq!(balances.balance(&bob), 220);
	}
}
