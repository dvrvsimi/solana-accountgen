use solana_account::Account;
use solana_pubkey::Pubkey;
use std::collections::HashMap;

/// A collection of accounts indexed by their pubkeys.
///
/// This struct provides a convenient way to manage multiple accounts
/// and their associated pubkeys.
#[derive(Debug, Default, Clone)]
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

    /// Creates a new AccountMap from an iterator of (Pubkey, Account) pairs.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountMap;
    /// use solana_pubkey::Pubkey;
    /// use solana_account::Account;
    ///
    /// let accounts = vec![
    ///     (Pubkey::new_unique(), Account::default()),
    ///     (Pubkey::new_unique(), Account::default()),
    /// ];
    ///
    /// let account_map = AccountMap::from_iter(accounts);
    /// assert_eq!(account_map.len(), 2);
    /// ```
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Pubkey, Account)>,
    {
        let mut map = Self::new();
        for (pubkey, account) in iter {
            map.set_account(pubkey, account);
        }
        map
    }

    /// Merges another AccountMap into this one.
    ///
    /// If both maps contain the same pubkey, the account from `other` will overwrite
    /// the existing account.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountMap;
    /// use solana_pubkey::Pubkey;
    /// use solana_account::Account;
    ///
    /// let mut map1 = AccountMap::new();
    /// let pubkey = Pubkey::new_unique();
    /// map1.set_account(pubkey, Account::default());
    ///
    /// let mut map2 = AccountMap::new();
    /// map2.set_account(Pubkey::new_unique(), Account::default());
    ///
    /// map1.merge(map2);
    /// assert_eq!(map1.len(), 2);
    /// ```
    pub fn merge(&mut self, other: AccountMap) {
        for (pubkey, account) in other {
            self.set_account(pubkey, account);
        }
    }

    /// Returns a new AccountMap containing only the accounts that satisfy the predicate.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountMap;
    /// use solana_pubkey::Pubkey;
    /// use solana_account::Account;
    ///
    /// let mut map = AccountMap::new();
    /// let pubkey1 = Pubkey::new_unique();
    /// let mut account1 = Account::default();
    /// account1.lamports = 100;
    /// map.set_account(pubkey1, account1);
    ///
    /// let pubkey2 = Pubkey::new_unique();
    /// let mut account2 = Account::default();
    /// account2.lamports = 200;
    /// map.set_account(pubkey2, account2);
    ///
    /// let filtered = map.filter(|_, account| account.lamports > 150);
    /// assert_eq!(filtered.len(), 1);
    /// ```
    pub fn filter<F>(&self, mut predicate: F) -> Self
    where
        F: FnMut(&Pubkey, &Account) -> bool,
    {
        let accounts = self
            .accounts
            .iter()
            .filter(|(pubkey, account)| predicate(pubkey, account))
            .map(|(pubkey, account)| (*pubkey, account.clone()))
            .collect::<HashMap<_, _>>();

        Self { accounts }
    }
}

impl IntoIterator for AccountMap {
    type Item = (Pubkey, Account);
    type IntoIter = std::collections::hash_map::IntoIter<Pubkey, Account>;

    fn into_iter(self) -> Self::IntoIter {
        self.accounts.into_iter()
    }
}

impl FromIterator<(Pubkey, Account)> for AccountMap {
    fn from_iter<I: IntoIterator<Item = (Pubkey, Account)>>(iter: I) -> Self {
        let mut map = Self::new();
        for (pubkey, account) in iter {
            map.set_account(pubkey, account);
        }
        map
    }
}
