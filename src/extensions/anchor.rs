//! Helpers for testing Anchor programs.
//!
//! This module provides utilities specifically designed for testing
//! Anchor programs, including support for:
//!
//! - Creating accounts with Anchor's 8-byte discriminator
//! - Creating PDAs with proper discriminators
//! - Building Anchor instructions with method discriminators
//! - Deserializing Anchor account data
//!
//! # Anchor Discriminators
//!
//!
//! - **Account discriminators**: First 8 bytes of SHA-256 hash of "account:{account_type}"
//! - **Instruction discriminators**: First 8 bytes of SHA-256 hash of "global:{method_name}"
//!
//!
//!
//! # Usage
//!
//! This module is particularly useful for:
//!
//! - Setting up test fixtures with properly formatted Anchor accounts
//! - Creating PDAs that match Anchor's expected format
//! - Building instructions that Anchor programs can properly decode
//! - Extracting account data from Anchor accounts for verification

use crate::{AccountBuilder, AccountGenError};
use sha2::{Digest, Sha256};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

/// Creates an account with Anchor's discriminator prefix.
///
/// In Anchor, account data typically starts with an 8-byte discriminator
/// that identifies the account type. This function automatically prepends
/// the correct discriminator based on the account type name.
///
/// # Arguments
///
/// * `account_type` - The name of the account type in your Anchor program
/// * `program_id` - The program ID that owns this account
/// * `data` - The account data (without discriminator)
/// * `lamports` - The balance in lamports for this account
///
/// # Returns
///
/// A fully initialized account with the proper Anchor discriminator
pub fn create_anchor_account<T: borsh::BorshSerialize>(
    account_type: &str,
    program_id: Pubkey,
    data: T,
    lamports: u64,
) -> Result<Account, AccountGenError> {
    // Calculate Anchor's discriminator
    let discriminator = get_account_discriminator(account_type);

    // Serialize the data
    let mut account_data = Vec::with_capacity(8 + borsh::to_vec(&data)?.len());
    account_data.extend_from_slice(&discriminator);
    account_data.extend_from_slice(&borsh::to_vec(&data)?);

    // Create the account
    AccountBuilder::new()
        .balance(lamports)
        .owner(program_id)
        .data_raw(account_data)
        .try_build()
}

/// Creates an Anchor instruction with the proper method discriminator.
///
/// In Anchor, instructions start with an 8-byte discriminator that identifies
/// the method being called. This function automatically prepends the correct
/// discriminator based on the method name.
///
/// # Arguments
///
/// * `program_id` - The program ID to call
/// * `method_name` - The name of the method in your Anchor program
/// * `accounts` - The accounts required by this instruction
/// * `data` - The instruction data (without discriminator)
///
/// # Example
///
/// ```
/// use solana_accountgen::extensions::anchor::create_anchor_instruction;
/// use solana_program::pubkey::Pubkey;
/// use solana_sdk::instruction::AccountMeta;
/// use borsh::{BorshSerialize, BorshDeserialize};
///
/// #[derive(BorshSerialize, BorshDeserialize)]
/// struct UpdateScoreArgs {
///     new_score: u64,
/// }
///
/// let program_id = Pubkey::new_unique();
/// let game_account = Pubkey::new_unique();
/// let player = Pubkey::new_unique();
///
/// let ix = create_anchor_instruction(
///     program_id,
///     "update_score",
///     vec![
///         AccountMeta::new(game_account, false),
///         AccountMeta::new_readonly(player, true),
///     ],
///     UpdateScoreArgs { new_score: 100 },
/// ).unwrap();
/// ```
pub fn create_anchor_instruction<T: borsh::BorshSerialize>(
    program_id: Pubkey,
    method_name: &str,
    accounts: Vec<AccountMeta>,
    data: T,
) -> Result<Instruction, AccountGenError> {
    // Calculate Anchor's method discriminator
    let discriminator = get_method_discriminator(method_name);

    // Serialize the data
    let mut instruction_data = Vec::with_capacity(8 + borsh::to_vec(&data)?.len());
    instruction_data.extend_from_slice(&discriminator);
    instruction_data.extend_from_slice(&borsh::to_vec(&data)?);

    Ok(Instruction {
        program_id,
        accounts,
        data: instruction_data,
    })
}

/// Deserializes an Anchor account, skipping the 8-byte discriminator.
///
/// This function extracts the account data from an Anchor account,
/// skipping the 8-byte discriminator at the beginning.
///
/// # Arguments
///
/// * `account` - The Anchor account to deserialize
///
/// # Example
///
/// ```
/// use solana_accountgen::extensions::anchor::deserialize_anchor_account;
/// use solana_program::pubkey::Pubkey;
/// use borsh::{BorshSerialize, BorshDeserialize};
///
/// #[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
/// struct GameState {
///     player: Pubkey,
///     score: u64,
/// }
///
/// // Assuming `account` is an Anchor account with GameState data
/// // let game_state: GameState = deserialize_anchor_account(&account).unwrap();
/// ```
pub fn deserialize_anchor_account<T: borsh::BorshDeserialize>(
    account: &Account,
) -> Result<T, AccountGenError> {
    if account.data.len() <= 8 {
        return Err(AccountGenError::DeserializationError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Account data too short for Anchor account",
        )));
    }

    // Skip the 8-byte discriminator
    borsh::from_slice(&account.data[8..]).map_err(|e| {
        AccountGenError::DeserializationError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e,
        ))
    })
}

/// Creates a PDA account with Anchor's discriminator prefix.
///
/// # Arguments
///
/// * `account_type` - The name of the account type in your Anchor program
/// * `program_id` - The program ID that owns this account
/// * `seeds` - The seeds used to derive the PDA
/// * `data` - The account data (without discriminator)
/// * `lamports` - The balance in lamports for this account
///
/// # Returns
///
/// A tuple containing the PDA pubkey, bump seed, and the created account
///
/// # Example
///
/// ```
/// use solana_accountgen::extensions::anchor::create_anchor_pda;
/// use solana_program::pubkey::Pubkey;
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
/// // Create a PDA with "game" discriminator
/// let (pda, bump, account) = create_anchor_pda(
///     "game",
///     program_id,
///     seeds,
///     game_state,
///     100_000, // lamports
/// ).unwrap();
/// ```
pub fn create_anchor_pda<T: borsh::BorshSerialize>(
    account_type: &str,
    program_id: Pubkey,
    seeds: &[&[u8]],
    data: T,
    lamports: u64,
) -> Result<(Pubkey, u8, Account), AccountGenError> {
    // Find the PDA
    let (pda, bump) = Pubkey::find_program_address(seeds, &program_id);

    // Calculate Anchor's discriminator
    let discriminator = get_account_discriminator(account_type);

    // Serialize the data
    let mut account_data = Vec::with_capacity(8 + borsh::to_vec(&data)?.len());
    account_data.extend_from_slice(&discriminator);
    account_data.extend_from_slice(&borsh::to_vec(&data)?);

    // Create the account
    let account = AccountBuilder::new()
        .balance(lamports)
        .owner(program_id)
        .data_raw(account_data)
        .try_build()?;

    Ok((pda, bump, account))
}

/// Calculates the Anchor account discriminator for a given account type.
///
/// The discriminator is the first 8 bytes of the SHA-256 hash of "account:{account_type}".
///
/// # Arguments
///
/// * `account_type` - The name of the account type in your Anchor program
///
/// # Returns
///
/// An 8-byte array containing the discriminator
pub fn get_account_discriminator(account_type: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("account:{}", account_type).as_bytes());
    let hash = hasher.finalize();
    hash[..8].try_into().unwrap()
}

/// Calculates the Anchor method discriminator for a given method name.
///
/// The discriminator is the first 8 bytes of the SHA-256 hash of "global:{method_name}".
///
/// # Arguments
///
/// * `method_name` - The name of the method in your Anchor program
///
/// # Returns
///
/// An 8-byte array containing the discriminator
pub fn get_method_discriminator(method_name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", method_name).as_bytes());
    let hash = hasher.finalize();
    hash[..8].try_into().unwrap()
}
