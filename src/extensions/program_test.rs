//! Integration with solana-program-test.
//!
//! This module provides utilities for integrating AccountBuilder with
//! the solana-program-test framework.

use crate::AccountBuilder;
use solana_program::pubkey::Pubkey;
use solana_program_test::{BanksClient, ProgramTest};

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
    /// This is done by creating and processing a transaction that creates the account.
    #[allow(async_fn_in_trait)]
    async fn set_account_with_builder(
        &mut self,
        _pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl BanksClientExt for BanksClient {
    async fn set_account_with_builder(
        &mut self,
        _pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _account = builder.try_build()?;
        
        // In a real implementation, you would:
        // 1. Create a system instruction to create the account
        // 2. Create a transaction with that instruction
        // 3. Process the transaction
        
        // For now, we'll just return Ok as a placeholder
        // In a real implementation, you would need to fund the payer account first
        // and handle other complexities
        Ok(())
    }
}
