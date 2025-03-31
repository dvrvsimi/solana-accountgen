//! Helpers for creating SPL Token accounts.
//!
//! This module provides utilities for creating mock SPL Token accounts
//! for testing purposes.

use crate::AccountBuilder;
use solana_program::pubkey::Pubkey;
use std::convert::TryFrom;

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
) -> Result<solana_sdk::account::Account, crate::error::AccountGenError> {
    // This is a simplified implementation
    // In a real implementation, we would create the proper SPL Token account structure
    
    let data = vec![
        // Simplified token account data structure
        // In reality, this would be properly serialized SPL Token account data
        1, // initialized
        0, 0, 0, 0, 0, 0, 0, 0, // padding
    ];
    
    AccountBuilder::new()
        .balance(1_000_000) // Rent exempt amount
        .owner(*token_program_id)
        .data_raw(data)
        .try_build()
} 