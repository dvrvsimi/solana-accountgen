//! Integration with solana-program-test.
//!
//! This module provides utilities for integrating AccountBuilder with
//! the solana-program-test framework.

use crate::AccountBuilder;
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::account::Account;
use std::convert::TryFrom;

/// Extension trait for ProgramTest to add accounts using AccountBuilder.
pub trait ProgramTestExt {
    /// Adds an account created with AccountBuilder to the test environment.
    fn add_account_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<&mut Self, crate::error::AccountGenError>;
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
}

/// Extension trait for BanksClient to add accounts using AccountBuilder.
pub trait BanksClientExt {
    /// Sets an account created with AccountBuilder in the test environment.
    fn set_account_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl BanksClientExt for BanksClient {
    fn set_account_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let account = builder.try_build()?;
        self.set_account(&pubkey, &account);
        Ok(())
    }
}

// Note: This implementation is simplified and would need to be expanded
// in a real implementation to properly work with the async nature of BanksClient 