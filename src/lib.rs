//! # solana-accountgen
//! 
//! A utility crate for generating mock Solana accounts for testing purposes.
//! 
//! ## Features
//! 
//! - Create accounts with custom balances, owners, and data using a fluent API
//! - Serialize account data using Borsh (with JSON support for the bincode module)
//! - Support for creating PDAs (Program Derived Addresses)
//! - Integration with solana-program-test for end-to-end testing
//! - Support for Anchor programs with discriminator handling
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
//!     .unwrap()
//!     .build();
//! ```
//!
//! ## Anchor Program Testing
//!
//! solana-accountgen provides special support for testing Anchor programs:
//!
//! ```rust,no_run
//! use solana_accountgen::extensions::anchor::{create_anchor_account, create_anchor_instruction};
//! use solana_program::pubkey::Pubkey;
//! use borsh::{BorshSerialize, BorshDeserialize};
//! use solana_sdk::instruction::AccountMeta;
//!
//! #[derive(BorshSerialize, BorshDeserialize)]
//! struct GameState { 
//!     player: Pubkey,
//!     score: u64,
//! }
//!
//! // Create an account with Anchor's discriminator
//! let program_id = Pubkey::new_unique();
//! let player = Pubkey::new_unique();
//! let game_state = GameState { 
//!     player,
//!     score: 100,
//! };
//!
//! let account = create_anchor_account(
//!     "game",  // Account type name in Anchor program
//!     program_id,
//!     game_state,
//!     100_000, // lamports
//! ).unwrap();
//!
//! // Create an instruction with Anchor's method discriminator
//! let ix = create_anchor_instruction(
//!     program_id,
//!     "update_score",  // Method name in Anchor program
//!     vec![
//!         AccountMeta::new(Pubkey::new_unique(), false),
//!         AccountMeta::new_readonly(player, true),
//!     ],
//!     42u64, // Instruction data
//! ).unwrap();
//! ```

mod account_builder;
mod account_map;
mod error;
pub mod extensions;
pub mod serialization;

pub use account_builder::AccountBuilder;
pub use account_map::AccountMap;
pub use error::AccountGenError;

// Re-export dependencies that users will likely need
pub use borsh;

use solana_program::pubkey::Pubkey;

#[cfg(test)]
mod tests {
    use super::*;
    use borsh::{BorshSerialize, BorshDeserialize};
    use solana_program::{pubkey::Pubkey, system_program};
    use solana_sdk::rent::Rent;
    use crate::serialization::borsh as borsh_serialization;
    use serde::{Serialize, Deserialize};
    use base64;
    use serde_json;

    // Test data structures
    #[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
    struct TestBorshData {
        value: u64,
        name: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestBincodeData {
        value: u64,
        name: String,
    }

    #[test]
    fn test_account_builder_basic() {
        let program_id = Pubkey::new_unique();
        let balance = 100_000_000;
        
        let account = AccountBuilder::new()
            .balance(balance)
            .owner(program_id)
            .build();
            
        assert_eq!(account.lamports, balance);
        assert_eq!(account.owner, program_id);
        assert_eq!(account.data.len(), 0);
        assert_eq!(account.executable, false);
    }
    
    #[test]
    fn test_account_builder_with_borsh_data() {
        let program_id = Pubkey::new_unique();
        let test_data = TestBorshData { 
            value: 42, 
            name: "Test Account".to_string() 
        };
        
        let account = AccountBuilder::new()
            .balance(100_000)
            .owner(program_id)
            .data(test_data.clone())
            .unwrap()
            .build();
            
        // Deserialize and verify the data
        let deserialized: TestBorshData = TestBorshData::try_from_slice(&account.data).unwrap();
        assert_eq!(deserialized, test_data);
    }
    
    #[test]
    fn test_account_builder_with_raw_data() {
        let program_id = Pubkey::new_unique();
        let raw_data = vec![1, 2, 3, 4, 5];
        
        let account = AccountBuilder::new()
            .balance(100_000)
            .owner(program_id)
            .data_raw(raw_data.clone())
            .build();
            
        assert_eq!(account.data, raw_data);
    }
    
    #[test]
    fn test_account_builder_executable() {
        let program_id = Pubkey::new_unique();
        
        let account = AccountBuilder::new()
            .balance(100_000)
            .owner(program_id)
            .executable(true)
            .build();
            
        assert_eq!(account.executable, true);
    }
    
    #[test]
    fn test_account_builder_rent_epoch() {
        let program_id = Pubkey::new_unique();
        let rent_epoch = 123;
        
        let account = AccountBuilder::new()
            .balance(100_000)
            .owner(program_id)
            .rent_epoch(rent_epoch)
            .build();
            
        assert_eq!(account.rent_epoch, rent_epoch);
    }
    
    #[test]
    fn test_try_build_with_missing_owner() {
        // Create an account without specifying an owner
        let result = AccountBuilder::new()
            .balance(100_000)
            .try_build();
            
        // Now it should succeed with the default system program owner
        assert!(result.is_ok());
        if let Ok(account) = result {
            assert_eq!(account.owner, system_program::id());
        }
    }
    
    #[test]
    fn test_create_pda() {
        let program_id = Pubkey::new_unique();
        let user = Pubkey::new_unique();
        let seeds = &[b"test", user.as_ref()];
        let test_data = TestBorshData { 
            value: 42, 
            name: "PDA Account".to_string() 
        };
        
        let (pda, bump, account) = AccountBuilder::create_pda(
            &program_id,
            seeds,
            100_000,
            test_data.clone()
        ).unwrap();
        
        // Verify PDA derivation
        let (expected_pda, expected_bump) = Pubkey::find_program_address(seeds, &program_id);
        assert_eq!(pda, expected_pda);
        assert_eq!(bump, expected_bump);
        
        // Verify account properties
        assert_eq!(account.owner, program_id);
        assert_eq!(account.lamports, 100_000);
        
        // Verify data
        let deserialized: TestBorshData = TestBorshData::try_from_slice(&account.data).unwrap();
        assert_eq!(deserialized, test_data);
    }
    
    #[test]
    fn test_borsh_serialization() {
        let program_id = Pubkey::new_unique();
        let test_data = TestBorshData { 
            value: 42, 
            name: "Test Account".to_string() 
        };
        
        let account = AccountBuilder::new()
            .balance(100_000)
            .owner(program_id)
            .data(test_data.clone())
            .unwrap()
            .build();
            
        let deserialized = borsh_serialization::deserialize_account_data::<TestBorshData>(&account).unwrap();
        assert_eq!(deserialized, test_data);
    }
    
    #[test]
    fn test_account_base64_encoding() {
        let program_id = Pubkey::new_unique();
        let balance = 100_000_000;
        let data = vec![1, 2, 3, 4, 5];
        
        // Create an account
        let account = AccountBuilder::new()
            .balance(balance)
            .owner(program_id)
            .data_raw(data.clone())
            .build();
            
        // Serialize with serde_json
        let account_bytes = serde_json::to_vec(&account).unwrap();
        
        // Encode with base64
        let base64_string = base64::encode(&account_bytes);
        
        // Decode from base64
        let decoded_bytes = base64::decode(&base64_string).unwrap();
        
        // Deserialize with serde_json
        let decoded_account: solana_sdk::account::Account = 
            serde_json::from_slice(&decoded_bytes).unwrap();
        
        // Verify the account was correctly round-tripped
        assert_eq!(account.lamports, decoded_account.lamports);
        assert_eq!(account.owner, decoded_account.owner);
        assert_eq!(account.data, decoded_account.data);
        assert_eq!(account.executable, decoded_account.executable);
        assert_eq!(account.rent_epoch, decoded_account.rent_epoch);
    }
    
    #[test]
    fn test_account_builder_with_pubkey() {
        let pubkey = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();
        
        let (account_pubkey, account) = AccountBuilder::new()
            .pubkey(pubkey)
            .balance(100_000)
            .owner(program_id)
            .build_with_pubkey();
        
        assert_eq!(account_pubkey, pubkey);
        assert_eq!(account.lamports, 100_000);
        assert_eq!(account.owner, program_id);
    }
    
    #[test]
    fn test_create_account_helper() {
        let pubkey = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();
        
        let (account_pubkey, account) = create_account(
            pubkey,
            AccountBuilder::new()
                .balance(100_000)
                .owner(program_id)
        ).unwrap();
        
        assert_eq!(account_pubkey, pubkey);
        assert_eq!(account.lamports, 100_000);
        assert_eq!(account.owner, program_id);
    }
    
    #[test]
    fn test_account_map() {
        let program_id = Pubkey::new_unique();
        let pubkey1 = Pubkey::new_unique();
        let pubkey2 = Pubkey::new_unique();
        
        let mut account_map = AccountMap::new();
        
        // Add accounts using different methods
        account_map.add_with_builder(
            pubkey1,
            AccountBuilder::new()
                .balance(100_000)
                .owner(program_id)
        ).unwrap();
        
        let account2 = AccountBuilder::new()
            .balance(200_000)
            .owner(program_id)
            .build();
        account_map.set_account(pubkey2, account2);
        
        // Test retrieval
        let account1 = account_map.get_account(&pubkey1).unwrap();
        assert_eq!(account1.lamports, 100_000);
        
        let account2 = account_map.get_account(&pubkey2).unwrap();
        assert_eq!(account2.lamports, 200_000);
        
        // Test iteration
        let mut total_lamports = 0;
        for (_, account) in account_map.iter() {
            total_lamports += account.lamports;
        }
        assert_eq!(total_lamports, 300_000);
        
        // Test length
        assert_eq!(account_map.len(), 2);
        
        // Test removal
        let removed = account_map.remove_account(&pubkey1).unwrap();
        assert_eq!(removed.lamports, 100_000);
        assert_eq!(account_map.len(), 1);
    }
    
    #[test]
    fn test_create_accounts() {
        let program_id = Pubkey::new_unique();
        let pubkey1 = Pubkey::new_unique();
        let pubkey2 = Pubkey::new_unique();
        
        let accounts = create_accounts(vec![
            (pubkey1, AccountBuilder::new().balance(100_000).owner(program_id)),
            (pubkey2, AccountBuilder::new().balance(200_000).owner(program_id)),
        ]).unwrap();
        
        assert_eq!(accounts.len(), 2);
        
        let account1 = accounts.get_account(&pubkey1).unwrap();
        assert_eq!(account1.lamports, 100_000);
        
        let account2 = accounts.get_account(&pubkey2).unwrap();
        assert_eq!(account2.lamports, 200_000);
    }
    
    #[test]
    fn test_account_builder_default_owner() {
        // Create an account without specifying an owner
        let account = AccountBuilder::new()
            .balance(100_000)
            .build();
            
        // Verify that the owner defaults to the system program
        assert_eq!(account.owner, system_program::id());
    }
    
    #[test]
    fn test_account_builder_default_balance() {
        // Create test data
        let test_data = TestBorshData { 
            value: 42, 
            name: "Test Account".to_string() 
        };
        
        // Create an account without specifying a balance
        let account = AccountBuilder::new()
            .owner(Pubkey::new_unique())
            .data(test_data.clone())
            .unwrap()
            .build();
            
        // Calculate the expected rent-exempt balance
        let rent = Rent::default();
        let data_size = borsh::to_vec(&test_data).unwrap().len();
        let expected_balance = rent.minimum_balance(data_size);
            
        // Verify that the balance defaults to rent-exempt
        assert_eq!(account.lamports, expected_balance);
    }
    
    #[test]
    fn test_account_builder_all_defaults() {
        // Create test data
        let test_data = TestBorshData { 
            value: 42, 
            name: "Test Account".to_string() 
        };
        
        // Create an account with only data specified
        let account = AccountBuilder::new()
            .data(test_data.clone())
            .unwrap()
            .build();
            
        // Verify defaults
        assert_eq!(account.owner, system_program::id());
        assert_eq!(account.executable, false);
        assert_eq!(account.rent_epoch, 0);
        
        // Calculate the expected rent-exempt balance
        let rent = Rent::default();
        let data_size = borsh::to_vec(&test_data).unwrap().len();
        let expected_balance = rent.minimum_balance(data_size);
            
        // Verify that the balance defaults to rent-exempt
        assert_eq!(account.lamports, expected_balance);
    }
    
    #[test]
    fn test_account_builder_explicit_overrides_defaults() {
        // Create test data
        let test_data = TestBorshData { 
            value: 42, 
            name: "Test Account".to_string() 
        };
        
        let custom_owner = Pubkey::new_unique();
        let custom_balance = 999_999;
        
        // Create an account with explicit values
        let account = AccountBuilder::new()
            .owner(custom_owner)
            .balance(custom_balance)
            .data(test_data.clone())
            .unwrap()
            .build();
            
        // Verify explicit values are used instead of defaults
        assert_eq!(account.owner, custom_owner);
        assert_eq!(account.lamports, custom_balance);
    }
}

/// Creates an account with the given pubkey and properties.
///
/// # Example
///
/// ```
/// use solana_accountgen::{create_account, AccountBuilder};
/// use solana_program::pubkey::Pubkey;
///
/// let pubkey = Pubkey::new_unique();
/// let program_id = Pubkey::new_unique();
/// let (account_pubkey, account) = create_account(
///     pubkey,
///     AccountBuilder::new()
///         .balance(100_000_000)
///         .owner(program_id)
/// ).unwrap();
/// ```
pub fn create_account(
    pubkey: Pubkey,
    builder: AccountBuilder,
) -> Result<(Pubkey, solana_sdk::account::Account), AccountGenError> {
    builder.pubkey(pubkey).try_build_with_pubkey()
}

/// Creates multiple accounts with their pubkeys.
///
/// # Example
///
/// ```
/// use solana_accountgen::{create_accounts, AccountBuilder};
/// use solana_program::pubkey::Pubkey;
///
/// let program_id = Pubkey::new_unique();
/// let accounts = create_accounts(vec![
///     (Pubkey::new_unique(), AccountBuilder::new().balance(100_000).owner(program_id)),
///     (Pubkey::new_unique(), AccountBuilder::new().balance(200_000).owner(program_id)),
/// ]).unwrap();
///
/// assert_eq!(accounts.len(), 2);
/// ```
pub fn create_accounts(
    accounts: Vec<(Pubkey, AccountBuilder)>,
) -> Result<AccountMap, AccountGenError> {
    let mut account_map = AccountMap::new();
    
    for (pubkey, builder) in accounts {
        account_map.add_with_builder(pubkey, builder)?;
    }
    
    Ok(account_map)
} 