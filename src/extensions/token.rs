//! Helpers for creating SPL Token accounts.
//!
//! This module provides utilities for creating mock SPL Token accounts
//! for testing purposes.

use crate::{AccountBuilder, AccountGenError};
use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
struct TokenAccount {
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Option<Pubkey>,
    state: u8,
    is_native: Option<u64>,
    delegated_amount: u64,
    close_authority: Option<Pubkey>,
}

/// Creates a mock SPL Token account with the given parameters.
///
/// # Example
///
/// ```
/// use solana_accountgen::extensions::token::create_token_account;
/// use solana_program::pubkey::Pubkey;
///
/// let mint = Pubkey::new_unique();
/// let owner = Pubkey::new_unique();
/// let token_program_id = Pubkey::new_unique();
/// 
/// let account = create_token_account(
///     &mint,
///     &owner,
///     1000,
///     &token_program_id,
/// ).unwrap();
/// ```
pub fn create_token_account(
    mint: &Pubkey,
    owner: &Pubkey,
    amount: u64,
    token_program_id: &Pubkey,
) -> Result<solana_sdk::account::Account, AccountGenError> {
    let token_account = TokenAccount {
        mint: *mint,
        owner: *owner,
        amount,
        delegate: None,
        state: 1, // initialized
        is_native: None,
        delegated_amount: 0,
        close_authority: None,
    };
    
    AccountBuilder::new()
        .balance(1_000_000) // Rent exempt amount
        .owner(*token_program_id)
        .data(token_account)?
        .try_build()
} 