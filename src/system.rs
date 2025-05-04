use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Ord + Copy + Zero + One + AddAssign;
	type Nonce: Ord + Copy + Zero + One;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	/// Get the nonce of an account.
	pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::zero())
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let current = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		self.nonce.insert(who.clone(), current + T::Nonce::one());
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		struct TestConfig;
		impl super::Config for TestConfig {
			type AccountId = String;
			type BlockNumber = u32;
			type Nonce = u32;
		}

		let mut system = super::Pallet::<TestConfig>::new();

		assert_eq!(system.block_number(), 0);

		system.inc_block_number();

		assert_eq!(system.block_number(), 1);

		assert_eq!(system.nonce(&"alice".to_string()), 0);

		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.nonce(&"alice".to_string()), 1);

		assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}
