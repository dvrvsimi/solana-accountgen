//! Bincode serialization support.
//!
//! This module provides utilities for working with Bincode serialization,
//! which is sometimes used for Solana account data.

use crate::error::AccountGenError;
use serde::{Deserialize, Serialize};
use solana_sdk::account::Account;
use std::io;

/// Deserializes account data using Bincode.
///
/// # Example
///
/// ```
/// use solana_accountgen::serialization::bincode::deserialize_account_data;
/// use serde::{Serialize, Deserialize};
/// use solana_sdk::account::Account;
/// use solana_program::pubkey::Pubkey;
///
/// #[derive(Serialize, Deserialize)]
/// struct MyData {
///     value: u64,
/// }
///
/// // Create an account with bincode-serialized data
/// let my_data = MyData { value: 42 };
/// let serialized = bincode::serialize(&my_data).unwrap();
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
pub fn deserialize_account_data<T: for<'de> Deserialize<'de>>(
    account: &Account,
) -> Result<T, AccountGenError> {
    bincode::decode_from_slice(&account.data, bincode::config::standard())
        .map(|(data, _)| data)
        .map_err(|e| AccountGenError::DeserializationError(io::Error::new(io::ErrorKind::InvalidData, e)))
}

/// Serializes data using Bincode.
///
/// # Example
///
/// ```
/// use solana_accountgen::serialization::bincode::serialize_data;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct MyData {
///     value: u64,
/// }
///
/// let my_data = MyData { value: 42 };
/// let serialized = serialize_data(&my_data).unwrap();
/// ```
pub fn serialize_data<T: Serialize>(data: &T) -> Result<Vec<u8>, AccountGenError> {
    bincode::encode_to_vec(data, bincode::config::standard())
        .map_err(|e| AccountGenError::SerializationError(io::Error::new(io::ErrorKind::InvalidData, e)))
} 