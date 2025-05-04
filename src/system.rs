use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	/// Get the nonce of an account.
	pub fn nonce(&self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&0)
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		self.nonce.insert(who.clone(), self.nonce.get(who).unwrap_or(&0) + 1);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		let mut system = super::Pallet::new();

		assert_eq!(system.block_number(), 0);

		system.inc_block_number();

		assert_eq!(system.block_number(), 1);

		assert_eq!(system.nonce(&"alice".to_string()), 0);

		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.nonce(&"alice".to_string()), 1);

		assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}
