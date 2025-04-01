use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use crate::error::AccountGenError;

/// A builder for creating mock Solana accounts for testing purposes.
///
/// This struct provides a fluent API for configuring and building
/// Solana accounts with custom properties.
#[derive(Debug, Default)]
pub struct AccountBuilder {
    lamports: u64,
    owner: Option<Pubkey>,
    executable: bool,
    rent_epoch: u64,
    data: Vec<u8>,
}

impl AccountBuilder {
    /// Creates a new `AccountBuilder` with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    ///
    /// let builder = AccountBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the account balance in lamports.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    ///
    /// let builder = AccountBuilder::new()
    ///     .balance(1_000_000);
    /// ```
    pub fn balance(mut self, lamports: u64) -> Self {
        self.lamports = lamports;
        self
    }

    /// Sets the account owner.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use solana_program::pubkey::Pubkey;
    ///
    /// let program_id = Pubkey::new_unique();
    /// let builder = AccountBuilder::new()
    ///     .owner(program_id);
    /// ```
    pub fn owner(mut self, owner: Pubkey) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Sets whether the account is executable.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    ///
    /// let builder = AccountBuilder::new()
    ///     .executable(true);
    /// ```
    pub fn executable(mut self, executable: bool) -> Self {
        self.executable = executable;
        self
    }

    /// Sets the account rent epoch.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    ///
    /// let builder = AccountBuilder::new()
    ///     .rent_epoch(100);
    /// ```
    pub fn rent_epoch(mut self, rent_epoch: u64) -> Self {
        self.rent_epoch = rent_epoch;
        self
    }

    /// Sets the account data using raw bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    ///
    /// let data = vec![0, 1, 2, 3];
    /// let builder = AccountBuilder::new()
    ///     .data_raw(data);
    /// ```
    pub fn data_raw(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }

    /// Sets the account data using a Borsh-serializable type.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use borsh::{BorshSerialize, BorshDeserialize};
    ///
    /// #[derive(BorshSerialize, BorshDeserialize)]
    /// struct MyData {
    ///     value: u64,
    /// }
    ///
    /// let my_data = MyData { value: 42 };
    /// let builder = AccountBuilder::new()
    ///     .data(my_data);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn data<T: BorshSerialize>(mut self, data: T) -> Result<Self, AccountGenError> {
        self.data = borsh::to_vec(&data).map_err(AccountGenError::SerializationError)?;
        Ok(self)
    }

    /// Builds the account with the configured properties.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use solana_program::pubkey::Pubkey;
    ///
    /// let program_id = Pubkey::new_unique();
    /// let account = AccountBuilder::new()
    ///     .balance(100_000_000)
    ///     .owner(program_id)
    ///     .build();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the owner has not been set.
    pub fn build(self) -> Account {
        Account {
            lamports: self.lamports,
            data: self.data,
            owner: self.owner.expect("Account owner must be set"),
            executable: self.executable,
            rent_epoch: self.rent_epoch,
        }
    }

    /// Attempts to build the account, returning an error if required fields are missing.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    ///
    /// let result = AccountBuilder::new()
    ///     .balance(100_000_000)
    ///     .try_build();
    /// assert!(result.is_err()); // Error because owner is not set
    /// ```
    pub fn try_build(self) -> Result<Account, AccountGenError> {
        let owner = self.owner.ok_or(AccountGenError::MissingOwner)?;
        
        Ok(Account {
            lamports: self.lamports,
            data: self.data,
            owner,
            executable: self.executable,
            rent_epoch: self.rent_epoch,
        })
    }

    /// Creates an account at a PDA with the given seeds
    pub fn create_pda(
        program_id: &Pubkey,
        seeds: &[&[u8]],
        balance: u64,
        data: impl BorshSerialize,
    ) -> Result<(Pubkey, u8, Account), AccountGenError> {
        let (pda, bump) = Pubkey::find_program_address(seeds, program_id);
        
        let account = Self::new()
            .balance(balance)
            .owner(*program_id)
            .data(data)?
            .try_build()?;
            
        Ok((pda, bump, account))
    }
} 