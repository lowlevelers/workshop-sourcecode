mod balances;
mod system;
mod types;

use types::{AddressNonce, Balance, BlockNumber};

use crate::types::Address;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl balances::Config for Runtime {
	type Balance = Balance;
}

impl system::Config for Runtime {
	type Address = Address;
	type BlockNumber = BlockNumber;
	type Nonce = AddressNonce;
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");
	runtime.balances.set_balance(&alice, 100);

	// start emulating a block
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| println!("{e:?}"));

	// second transaction
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| println!("{e:?}"));
}
