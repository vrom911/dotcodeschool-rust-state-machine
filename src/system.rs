use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
	AccountId: Ord + Clone,
	BlockNumber: Ord + Copy + Zero + One + AddAssign,
	Nonce: Ord + Copy + Zero + One,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	/// Get the nonce of an account.
	pub fn nonce(&self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&Nonce::zero())
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		let current = *self.nonce.get(who).unwrap_or(&Nonce::zero());
		self.nonce.insert(who.clone(), current + Nonce::one());
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::<String, u32, u32>::new();

		assert_eq!(system.block_number(), 0);

		system.inc_block_number();

		assert_eq!(system.block_number(), 1);

		assert_eq!(system.nonce(&"alice".to_string()), 0);

		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.nonce(&"alice".to_string()), 1);

		assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}
