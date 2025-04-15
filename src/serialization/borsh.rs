//! Borsh serialization support.
//!
//! This module provides utilities for working with Borsh serialization,
//! which is the primary serialization format used by Solana programs.

use crate::error::AccountGenError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_account::Account;

/// Deserializes account data using Borsh.
///
/// # Example
///
/// ```
/// use solana_accountgen::serialization::borsh::deserialize_account_data;
/// use borsh::{BorshSerialize, BorshDeserialize};
/// use solana_account::Account;
/// use solana_pubkey::Pubkey;
///
/// #[derive(BorshSerialize, BorshDeserialize)]
/// struct MyData {
///     value: u64,
/// }
///
/// // Create an account with borsh-serialized data
/// let my_data = MyData { value: 42 };
/// let serialized = borsh::to_vec(&my_data).unwrap();
/// let account = Account {
///     lamports: 100,
///     data: serialized,
///     owner: Pubkey::new_unique(),
///     executable: false,
///     rent_epoch: 0,
/// };
///
/// let deserialized: MyData = deserialize_account_data(&account).unwrap();
/// assert_eq!(deserialized.value, 42);
/// ```
pub fn deserialize_account_data<T: BorshDeserialize>(
    account: &Account,
) -> Result<T, AccountGenError> {
    T::try_from_slice(&account.data).map_err(AccountGenError::DeserializationError)
}

/// Serializes data using Borsh.
///
/// # Example
///
/// ```
/// use solana_accountgen::serialization::borsh::serialize_data;
/// use borsh::{BorshSerialize, BorshDeserialize};
///
/// #[derive(BorshSerialize, BorshDeserialize)]
/// struct MyData {
///     value: u64,
/// }
///
/// let my_data = MyData { value: 42 };
/// let serialized = serialize_data(&my_data).unwrap();
/// ```
pub fn serialize_data<T: BorshSerialize>(data: &T) -> Result<Vec<u8>, AccountGenError> {
    borsh::to_vec(data).map_err(|e| AccountGenError::SerializationError(e))
}
