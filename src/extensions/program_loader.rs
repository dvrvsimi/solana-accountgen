//! Utilities for loading program files.
//!
//! This module provides functions for loading SBF program files
//! and creating executable accounts.

use crate::{AccountBuilder, AccountGenError};
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

/// Finds a program file in the default search paths.
pub fn find_program_file(filename: &str) -> Option<PathBuf> {
    for dir in default_program_dirs() {
        let candidate = dir.join(filename);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

/// Returns the default directories to search for program files.
fn default_program_dirs() -> Vec<PathBuf> {
    let mut search_path = vec![];
    if let Ok(bpf_out_dir) = std::env::var("BPF_OUT_DIR") {
        search_path.push(PathBuf::from(bpf_out_dir));
    } else if let Ok(sbf_out_dir) = std::env::var("SBF_OUT_DIR") {
        search_path.push(PathBuf::from(sbf_out_dir));
    }
    search_path.push(PathBuf::from("target/deploy"));
    search_path.push(PathBuf::from("tests/fixtures"));
    if let Ok(dir) = std::env::current_dir() {
        search_path.push(dir);
    }
    search_path
}

/// Reads a file into a byte vector.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;
    Ok(file_data)
}

/// Creates an executable program account from a file.
///
/// # Example
///
/// ```
/// use solana_accountgen::extensions::program_loader::create_program_account_from_file;
/// use solana_pubkey::Pubkey;
///
/// let program_id = Pubkey::new_unique();
/// let program_account = create_program_account_from_file(
///     "my_program.so",
///     &solana_sdk::bpf_loader::id(),
/// ).unwrap();
/// ```
pub fn create_program_account_from_file(
    program_filename: &str,
    program_owner: &Pubkey,
) -> Result<Account, AccountGenError> {
    let program_file = find_program_file(program_filename)
        .ok_or_else(|| AccountGenError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Program file not found: {}", program_filename),
        )))?;
    
    let program_data = read_file(program_file)
        .map_err(AccountGenError::IoError)?;
    
    AccountBuilder::new()
        .balance(solana_sdk::rent::Rent::default().minimum_balance(program_data.len()))
        .owner(*program_owner)
        .data_raw(program_data)
        .executable(true)
        .try_build()
} 