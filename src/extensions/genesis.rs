//! Helpers for creating genesis accounts.
//!
//! This module provides utilities for creating accounts that should be
//! included in the genesis config.

use crate::AccountMap;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;

/// A collection of accounts to be included in genesis.
#[derive(Debug, Default)]
pub struct GenesisAccounts {
    accounts: AccountMap,
}

impl GenesisAccounts {
    /// Creates a new empty `GenesisAccounts`.
    pub fn new() -> Self {
        Self {
            accounts: AccountMap::new(),
        }
    }
    
    /// Adds an account to the genesis accounts.
    pub fn add_account(&mut self, pubkey: Pubkey, account: Account) -> &mut Self {
        self.accounts.set_account(pubkey, account);
        self
    }
    
    /// Adds all accounts from an AccountMap to the genesis accounts.
    pub fn add_account_map(&mut self, account_map: AccountMap) -> &mut Self {
        for (pubkey, account) in account_map {
            self.accounts.set_account(pubkey, account);
        }
        self
    }
    
    /// Returns an iterator over all (pubkey, account) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&Pubkey, &Account)> {
        self.accounts.iter()
    }
    
    /// Returns the number of accounts.
    pub fn len(&self) -> usize {
        self.accounts.len()
    }
    
    /// Returns true if there are no accounts.
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }
}

impl IntoIterator for GenesisAccounts {
    type Item = (Pubkey, Account);
    type IntoIter = std::collections::hash_map::IntoIter<Pubkey, Account>;

    fn into_iter(self) -> Self::IntoIter {
        self.accounts.into_iter()
    }
} 