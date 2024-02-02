#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod impls;
mod types;

#[openbrush::implementation(PSP34, PSP34Mintable, PSP34Metadata, PSP34Enumerable, Ownable)]
#[openbrush::contract]
mod psp34_marketplace_example {
	use crate::impls;
	use ink::codegen::{EmitEvent, Env};
	use openbrush::{
		contracts::{
			psp34::{extensions::metadata, PSP34Impl},
			reentrancy_guard,
		},
		traits::Storage,
	};

	#[ink(event)]
	pub struct Transfer {
		#[ink(topic)]
		from: Option<AccountId>,
		#[ink(topic)]
		to: Option<AccountId>,
		#[ink(topic)]
		id: Id,
	}

	#[ink(event)]
	pub struct Approval {
		#[ink(topic)]
		from: AccountId,
		#[ink(topic)]
		to: AccountId,
		#[ink(topic)]
		id: Option<Id>,
		approved: bool,
	}

	// Override event emission methods
	#[overrider(psp34::Internal)]
	fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
		self.env().emit_event(Transfer { from, to, id });
	}

	#[overrider(psp34::Internal)]
	fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
		self.env().emit_event(Approval { from, to, id, approved });
	}

	#[ink(storage)]
	#[derive(Default, Storage)]
	pub struct SpaceMarket {
		#[storage_field]
		psp34: psp34::Data,
		#[storage_field]
		guard: reentrancy_guard::Data,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		metadata: metadata::Data,
		#[storage_field]
		nftdata: crate::types::NftData,
		#[storage_field]
		enumerable: enumerable::Data,
	}

	impl impls::NftMarketImpl for SpaceMarket {}
	impl impls::MarketInternal for SpaceMarket {}

	impl SpaceMarket {
		fn set_colllection_attributes(&mut self, name: String, symbol: String, base_uri: String) {
			let col_id = PSP34Impl::collection_id(self);
			metadata::InternalImpl::_set_attribute(
				self,
				col_id.clone(),
				String::from("name"),
				name,
			);
			metadata::InternalImpl::_set_attribute(
				self,
				col_id.clone(),
				String::from("symbol"),
				symbol,
			);
			metadata::InternalImpl::_set_attribute(self, col_id, String::from("baseUri"), base_uri);
		}

		#[ink(constructor)]
		pub fn new(
			name: String,
			symbol: String,
			base_uri: String,
			max_supply: u64,
			price_per_mint: Balance,
		) -> Self {
			let mut instance = Self::default();
			let caller = instance.env().caller();
			ownable::InternalImpl::_init_with_owner(&mut instance, caller);
			instance.set_colllection_attributes(name, symbol, base_uri);

			instance.nftdata.max_supply = max_supply;
			instance.nftdata.price_per_mint = price_per_mint;
			instance
		}
	}
}
