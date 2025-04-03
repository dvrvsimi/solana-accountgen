use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use std::collections::HashMap;

/// A collection of accounts indexed by their pubkeys.
///
/// This struct provides a convenient way to manage multiple accounts
/// and their associated pubkeys.
#[derive(Debug, Default)]
pub struct AccountMap {
    accounts: HashMap<Pubkey, Account>,
}

impl AccountMap {
    /// Creates a new empty `AccountMap`.
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    /// Adds an account with its pubkey to the map.
    pub fn set_account(&mut self, pubkey: Pubkey, account: Account) {
        self.accounts.insert(pubkey, account);
    }

    /// Adds an account created with `AccountBuilder` to the map.
    pub fn add_with_builder(
        &mut self,
        pubkey: Pubkey,
        builder: crate::AccountBuilder,
    ) -> Result<&mut Self, crate::AccountGenError> {
        let account = builder.try_build()?;
        self.accounts.insert(pubkey, account);
        Ok(self)
    }

    /// Gets a reference to an account by its pubkey.
    pub fn get_account(&self, pubkey: &Pubkey) -> Option<&Account> {
        self.accounts.get(pubkey)
    }

    /// Gets a mutable reference to an account by its pubkey.
    pub fn get_account_mut(&mut self, pubkey: &Pubkey) -> Option<&mut Account> {
        self.accounts.get_mut(pubkey)
    }

    /// Removes an account from the map.
    pub fn remove_account(&mut self, pubkey: &Pubkey) -> Option<Account> {
        self.accounts.remove(pubkey)
    }

    /// Returns an iterator over all (pubkey, account) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&Pubkey, &Account)> {
        self.accounts.iter()
    }

    /// Returns the number of accounts in the map.
    pub fn len(&self) -> usize {
        self.accounts.len()
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }
}

impl IntoIterator for AccountMap {
    type Item = (Pubkey, Account);
    type IntoIter = std::collections::hash_map::IntoIter<Pubkey, Account>;

    fn into_iter(self) -> Self::IntoIter {
        self.accounts.into_iter()
    }
} 