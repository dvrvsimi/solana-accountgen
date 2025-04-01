//! Bincode serialization support.
//!
//! This module provides utilities for working with Bincode serialization,
//! which is sometimes used for Solana account data.

use crate::error::AccountGenError;
use serde::{Deserialize, Serialize};
use solana_sdk::account::Account;
use std::io;
use bincode::{config::standard, Decode, Encode};

/// Deserializes account data using Bincode.
///
/// # Example
///
/// ```
/// use solana_accountgen::serialization::bincode::deserialize_account_data;
/// use serde::{Serialize, Deserialize};
/// use solana_sdk::account::Account;
/// use solana_program::pubkey::Pubkey;
/// use bincode::{Encode, Decode};
///
/// #[derive(Serialize, Deserialize)]
/// struct MyData {
///     value: u64,
/// }
///
/// // Implement Encode and Decode for MyData
/// impl Encode for MyData {
///     fn encode<E: bincode::enc::Encoder>(
///         &self,
///         encoder: &mut E,
///     ) -> Result<(), bincode::error::EncodeError> {
///         bincode::Encode::encode(&self.value, encoder)?;
///         Ok(())
///     }
/// }
///
/// impl Decode<()> for MyData {
///     fn decode<D: bincode::de::Decoder>(
///         decoder: &mut D,
///     ) -> Result<Self, bincode::error::DecodeError> {
///         let value = bincode::Decode::decode(decoder)?;
///         Ok(MyData { value })
///     }
/// }
///
/// // Create an account with bincode-serialized data
/// let my_data = MyData { value: 42 };
/// let serialized = bincode::encode_to_vec(&my_data, bincode::config::standard()).unwrap();
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
pub fn deserialize_account_data<T: for<'de> Deserialize<'de> + Decode<()>>(
    account: &Account,
) -> Result<T, AccountGenError> {
    bincode::decode_from_slice(&account.data, standard())
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
/// use bincode::{Encode, Decode};
///
/// #[derive(Serialize, Deserialize)]
/// struct MyData {
///     value: u64,
/// }
///
/// // Implement Encode and Decode for MyData
/// impl Encode for MyData {
///     fn encode<E: bincode::enc::Encoder>(
///         &self,
///         encoder: &mut E,
///     ) -> Result<(), bincode::error::EncodeError> {
///         bincode::Encode::encode(&self.value, encoder)?;
///         Ok(())
///     }
/// }
///
/// impl Decode<()> for MyData {
///     fn decode<D: bincode::de::Decoder>(
///         decoder: &mut D,
///     ) -> Result<Self, bincode::error::DecodeError> {
///         let value = bincode::Decode::decode(decoder)?;
///         Ok(MyData { value })
///     }
/// }
///
/// let my_data = MyData { value: 42 };
/// let serialized = serialize_data(&my_data).unwrap();
/// ```
pub fn serialize_data<T: Serialize + Encode>(data: &T) -> Result<Vec<u8>, AccountGenError> {
    bincode::encode_to_vec(data, standard())
        .map_err(|e| AccountGenError::SerializationError(io::Error::new(io::ErrorKind::InvalidData, e)))
} 