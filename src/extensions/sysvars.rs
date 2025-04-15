//! Helpers for creating Solana sysvar accounts.
//!
//! This module provides utilities for creating mock sysvar accounts
//! for testing purposes.

use crate::AccountBuilder;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    account::Account,
    sysvar::{Sysvar, SysvarId},
};

/// Creates a sysvar account with the given data.
///
/// # Example
///
/// ```
/// use solana_accountgen::extensions::sysvars::create_sysvar_account;
/// use solana_rent::Rent;
/// use solana_clock::Clock;
///
/// let clock = Clock::default();
/// let clock_account = create_sysvar_account(&clock);
///
/// let rent = Rent::default();
/// let rent_account = create_sysvar_account(&rent);
/// ```
pub fn create_sysvar_account<S: Sysvar + SysvarId>(sysvar: &S) -> Account {
    let mut account = Account::new(1, S::size_of(), &S::id());
    sysvar.to_account_data(&mut account.data).unwrap();
    account
} 