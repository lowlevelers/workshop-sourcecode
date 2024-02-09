mod balances;
mod proof_of_existence;
mod support;
mod system;

mod types {
	use crate::{support, RuntimeCall};

	pub type Address = String;
	pub type AddressNonce = u32;
	pub type BlockNumber = u32;
	pub type Balance = u128;

	pub type Extrinsic = support::Extrinsic<Address, RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

use support::{Dispatch, DispatchResult};
use types::*;

use crate::support::{Block, Header};

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl proof_of_existence::Config for Runtime {
	type Content = Content;
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
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
			proof_of_existence: proof_of_existence::Pallet::new(),
		}
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if self.system.block_number() != block.header.block_number {
			return Err("invalid block number");
		}
		let mut i = 0;
		for extrinsic in block.extrinsics {
			self.system.inc_nonce(&extrinsic.caller);

			let (caller, call) = (extrinsic.caller, extrinsic.call);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});

			i += 1;
		}
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::Address;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
			RuntimeCall::Balances(call) => self.balances.dispatch(caller, call),
			RuntimeCall::ProofOfExistence(call) => self.proof_of_existence.dispatch(caller, call),
		}
	}
}

fn main() -> DispatchResult {
	let mut runtime = Runtime::new();
	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie = String::from("charlie");
	runtime.balances.set_balance(&alice, 100);

	let genesis_block: types::Block =
		Block { header: Header { block_number: 1 }, extrinsics: vec![] };
	runtime.execute_block(genesis_block)?;

	let block_1: types::Block = Block {
		header: Header { block_number: 2 },
		extrinsics: vec![
			Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::BalanceTransfer {
					to: bob.clone(),
					amount: 30,
				}),
			},
			Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::BalanceTransfer {
					to: charlie.clone(),
					amount: 20,
				}),
			},
		],
	};
	runtime.execute_block(block_1)?;

	assert_eq!(runtime.balances.balance(&alice.clone()), 100 - 30 - 20);
	assert_eq!(runtime.balances.balance(&bob.clone()), 30);
	assert_eq!(runtime.balances.balance(&charlie.clone()), 20);

	let block_2: types::Block = Block {
		header: Header { block_number: 3 },
		extrinsics: vec![Extrinsic {
			caller: alice.clone(),
			call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
				claim: "document from alice",
			}),
		}],
	};
	runtime.execute_block(block_2)?;

	assert_eq!(runtime.proof_of_existence.get_claim(&"document from alice"), Some(&alice.clone()));
	Ok(())
}
