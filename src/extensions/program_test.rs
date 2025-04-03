//! Integration with solana-program-test.
//!
//! This module provides utilities for integrating AccountBuilder with
//! the solana-program-test framework, making it easier to set up test
//! environments for Solana programs.
//!
//! The `ProgramTestExt` trait extends Solana's `ProgramTest` with methods
//! that work with solana-accountgen's `AccountBuilder` and `AccountMap`,
//! as well as Anchor-specific account creation.

use crate::{AccountBuilder, AccountGenError, AccountMap};
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use crate::extensions::anchor;

/// Extension trait for ProgramTest to add accounts using AccountBuilder.
///
/// This trait extends Solana's `ProgramTest` with methods that make it
/// easier to add accounts created with solana-accountgen.
pub trait ProgramTestExt {
    /// Adds an account to the test environment using an AccountBuilder.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::{AccountBuilder, extensions::program_test::ProgramTestExt};
    /// use solana_program::pubkey::Pubkey;
    /// use solana_program_test::ProgramTest;
    ///
    /// let program_id = Pubkey::new_unique();
    /// let account_pubkey = Pubkey::new_unique();
    ///
    /// let mut program_test = ProgramTest::default();
    /// program_test.add_account_with_builder(
    ///     account_pubkey,
    ///     AccountBuilder::new()
    ///         .balance(1_000_000)
    ///         .owner(program_id)
    /// ).unwrap();
    /// ```
    fn add_account_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<&mut Self, AccountGenError>;

    /// Adds multiple accounts to the test environment using an AccountMap.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::{AccountBuilder, AccountMap, extensions::program_test::ProgramTestExt};
    /// use solana_program::pubkey::Pubkey;
    /// use solana_program_test::ProgramTest;
    ///
    /// let program_id = Pubkey::new_unique();
    /// let mut account_map = AccountMap::new();
    ///
    /// // Add accounts to the map
    /// account_map.add_with_builder(
    ///     Pubkey::new_unique(),
    ///     AccountBuilder::new().balance(1_000_000).owner(program_id)
    /// ).unwrap();
    ///
    /// // Add all accounts to the test environment
    /// let mut program_test = ProgramTest::default();
    /// program_test.add_account_map(account_map);
    /// ```
    fn add_account_map(
        &mut self,
        account_map: AccountMap,
    ) -> &mut Self;

    /// Adds an Anchor account with discriminator to the test environment.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::extensions::program_test::ProgramTestExt;
    /// use solana_program::pubkey::Pubkey;
    /// use solana_program_test::ProgramTest;
    /// use borsh::{BorshSerialize, BorshDeserialize};
    ///
    /// #[derive(BorshSerialize, BorshDeserialize)]
    /// struct GameState {
    ///     player: Pubkey,
    ///     score: u64,
    /// }
    ///
    /// let program_id = Pubkey::new_unique();
    /// let account_pubkey = Pubkey::new_unique();
    /// let game_state = GameState {
    ///     player: Pubkey::new_unique(),
    ///     score: 100,
    /// };
    ///
    /// let mut program_test = ProgramTest::default();
    /// program_test.add_anchor_account(
    ///     account_pubkey,
    ///     "game_account",
    ///     program_id,
    ///     game_state,
    ///     10_000_000,
    /// ).unwrap();
    /// ```
    fn add_anchor_account<T: borsh::BorshSerialize>(
        &mut self,
        pubkey: Pubkey,
        account_type: &str,
        program_id: Pubkey,
        data: T,
        lamports: u64,
    ) -> Result<&mut Self, AccountGenError>;

    /// Adds an Anchor PDA account with discriminator to the test environment.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::extensions::program_test::ProgramTestExt;
    /// use solana_program::pubkey::Pubkey;
    /// use solana_program_test::ProgramTest;
    /// use borsh::{BorshSerialize, BorshDeserialize};
    ///
    /// #[derive(BorshSerialize, BorshDeserialize)]
    /// struct GameState {
    ///     player: Pubkey,
    ///     score: u64,
    /// }
    ///
    /// let program_id = Pubkey::new_unique();
    /// let player = Pubkey::new_unique();
    /// let seeds = &[b"game", player.as_ref()];
    ///
    /// let game_state = GameState {
    ///     player,
    ///     score: 100,
    /// };
    ///
    /// let mut program_test = ProgramTest::default();
    /// let (pda, bump, _) = program_test.add_anchor_pda(
    ///     "game",
    ///     program_id,
    ///     seeds,
    ///     game_state,
    ///     10_000_000,
    /// ).unwrap();
    /// ```
    fn add_anchor_pda<T: borsh::BorshSerialize>(
        &mut self,
        account_type: &str,
        program_id: Pubkey,
        seeds: &[&[u8]],
        data: T,
        lamports: u64,
    ) -> Result<(Pubkey, u8, &mut Self), AccountGenError>;
}

impl ProgramTestExt for ProgramTest {
    fn add_account_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: AccountBuilder,
    ) -> Result<&mut Self, AccountGenError> {
        let account = builder.try_build()?;
        self.add_account(pubkey, account);
        Ok(self)
    }

    fn add_account_map(
        &mut self,
        account_map: AccountMap,
    ) -> &mut Self {
        for (pubkey, account) in account_map {
            self.add_account(pubkey, account);
        }
        self
    }

    fn add_anchor_account<T: borsh::BorshSerialize>(
        &mut self,
        pubkey: Pubkey,
        account_type: &str,
        program_id: Pubkey,
        data: T,
        lamports: u64,
    ) -> Result<&mut Self, AccountGenError> {
        let account = anchor::create_anchor_account(
            account_type,
            program_id,
            data,
            lamports,
        )?;
        self.add_account(pubkey, account);
        Ok(self)
    }

    fn add_anchor_pda<T: borsh::BorshSerialize>(
        &mut self,
        account_type: &str,
        program_id: Pubkey,
        seeds: &[&[u8]],
        data: T,
        lamports: u64,
    ) -> Result<(Pubkey, u8, &mut Self), AccountGenError> {
        let (pda, bump, account) = anchor::create_anchor_pda(
            account_type,
            program_id,
            seeds,
            data,
            lamports,
        )?;
        self.add_account(pda, account);
        Ok((pda, bump, self))
    }
}
