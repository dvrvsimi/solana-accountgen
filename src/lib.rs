//! # solana-accountgen
//! 
//! A utility crate for generating mock Solana accounts for testing purposes.
//! 
//! ## Features
//! 
//! - Create accounts with custom balances, owners, and data using a fluent API
//! - Serialize account data using Borsh (with Bincode support planned)
//! - Support for creating PDAs (Program Derived Addresses)
//! - Integration with solana-program-test for end-to-end testing
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use solana_accountgen::AccountBuilder;
//! use solana_program::pubkey::Pubkey;
//! use borsh::{BorshSerialize, BorshDeserialize};
//! 
//! #[derive(BorshSerialize, BorshDeserialize)]
//! struct MyData { value: u64 }
//! 
//! let program_id = Pubkey::new_unique();
//! let account = AccountBuilder::new()
//!     .balance(100_000_000)
//!     .owner(program_id)
//!     .data(MyData { value: 42 })
//!     .build();
//! ```

mod account_builder;
mod error;
pub mod extensions;
pub mod serialization;

pub use account_builder::AccountBuilder;
pub use error::AccountGenError;

// Re-export dependencies that users will likely need
pub use borsh; 