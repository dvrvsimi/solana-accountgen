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
//!     .unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;
    use borsh::{BorshSerialize, BorshDeserialize};
    use solana_program::pubkey::Pubkey;
    use solana_sdk::account::Account;
    use crate::serialization::{borsh as borsh_serialization, bincode as bincode_serialization};
    use serde::{Serialize, Deserialize};

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

    // Implement Encode and Decode for TestBincodeData
    impl bincode::Encode for TestBincodeData {
        fn encode<E: bincode::enc::Encoder>(
            &self,
            encoder: &mut E,
        ) -> Result<(), bincode::error::EncodeError> {
            bincode::Encode::encode(&self.value, encoder)?;
            bincode::Encode::encode(&self.name, encoder)?;
            Ok(())
        }
    }

    impl bincode::Decode<()> for TestBincodeData {
        fn decode<D: bincode::de::Decoder>(
            decoder: &mut D,
        ) -> Result<Self, bincode::error::DecodeError> {
            let value = bincode::Decode::decode(decoder)?;
            let name = bincode::Decode::decode(decoder)?;
            Ok(TestBincodeData { value, name })
        }
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
        let result = AccountBuilder::new()
            .balance(100_000)
            .try_build();
            
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(matches!(err, AccountGenError::MissingOwner));
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
    fn test_bincode_serialization() {
        let program_id = Pubkey::new_unique();
        let test_data = TestBincodeData { 
            value: 42, 
            name: "Test Account".to_string() 
        };
        
        // Serialize with bincode
        let serialized = bincode_serialization::serialize_data(&test_data).unwrap();
        
        let account = Account {
            lamports: 100_000,
            data: serialized,
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        };
            
        let deserialized = bincode_serialization::deserialize_account_data::<TestBincodeData>(&account).unwrap();
        assert_eq!(deserialized, test_data);
    }
    
    // Test for extensions could be added here, but they would likely need
    // more complex setup with solana-program-test which is better suited
    // for integration tests
} 