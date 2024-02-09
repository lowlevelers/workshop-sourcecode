use std::{collections::BTreeMap, fmt::Debug};

use crate::{
	support::{DispatchGenericResult, DispatchResult},
	system,
};

pub trait Config: system::Config {
	type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	// content -> address
	claims: BTreeMap<T::Content, T::Address>,
}

pub enum Call<T: Config> {
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::Address;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
			Call::CreateClaim { claim } => self.create_claim(caller, claim),
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
		}
	}
}

impl<T> Pallet<T>
where
	T: Config,
{
	pub fn new() -> Self {
		Self { claims: BTreeMap::default() }
	}

	pub fn get_claim(&mut self, claim: &T::Content) -> Option<&T::Address> {
		return self.claims.get(claim);
	}

	pub fn create_claim(&mut self, caller: T::Address, claim: T::Content) -> DispatchResult {
		if self.get_claim(&claim).is_some() {
			return Err("this content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::Address, claim: T::Content) -> DispatchResult {
		let get_claim_res = || -> DispatchGenericResult<T::Content> {
			match self.get_claim(&claim) {
				Some(address) => {
					if caller != *address {
						return Err("invalid claim ownership");
					}
					return Ok(claim);
				},
				None => Err("claim does not exist"),
			}
		};

		let found_claim = get_claim_res()?;
		self.claims.remove(&found_claim);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type Address = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"Hello, world!"), None);
		assert_eq!(poe.create_claim(&"alice", &"Hello, world!"), Ok(()));
		assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			poe.create_claim(&"bob", &"Hello, world!"),
			Err("this content is already claimed")
		);
		assert_eq!(poe.revoke_claim(&"alice", &"Hello, world!"), Ok(()));
		assert_eq!(poe.create_claim(&"bob", &"Hello, world!"), Ok(()));
	}
}
