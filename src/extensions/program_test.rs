//! Integration with solana-program-test.
//!
//! This module provides utilities for integrating AccountBuilder with
//! the solana-program-test framework.
//!
//! Note: Accounts should be added to ProgramTest before starting the test.
//! Once the test has started, adding new accounts is not directly supported.

use crate::AccountBuilder;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use crate::account_map::AccountMap;

/// Extension trait for ProgramTest to add accounts using AccountBuilder.
pub trait ProgramTestExt {
    /// Adds an account created with AccountBuilder to the test environment.
    fn add_account_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<&mut Self, crate::error::AccountGenError>;
    
    /// Adds multiple accounts created with AccountBuilder to the test environment.
    fn add_accounts(
        &mut self,
        accounts: Vec<(Pubkey, AccountBuilder)>,
    ) -> Result<&mut Self, crate::error::AccountGenError>;

    /// Adds all accounts from an AccountMap to the test environment.
    fn add_account_map(
        &mut self,
        account_map: AccountMap,
    ) -> &mut Self;
}

impl ProgramTestExt for ProgramTest {
    fn add_account_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<&mut Self, crate::error::AccountGenError> {
        let account = builder.try_build()?;
        self.add_account(pubkey, account);
        Ok(self)
    }
    
    fn add_accounts(
        &mut self,
        accounts: Vec<(Pubkey, AccountBuilder)>,
    ) -> Result<&mut Self, crate::error::AccountGenError> {
        for (pubkey, builder) in accounts {
            self.add_account_with_builder(pubkey, builder)?;
        }
        Ok(self)
    }

    /// Adds all accounts from an AccountMap to the test environment.
    fn add_account_map(
        &mut self,
        account_map: AccountMap,
    ) -> &mut Self {
        for (pubkey, account) in account_map {
            self.add_account(pubkey, account);
        }
        self
    }
}
