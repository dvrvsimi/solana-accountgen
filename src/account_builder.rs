use crate::error::AccountGenError;
use base64;
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};
use solana_account::Account;
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_sdk_ids::system_program;

/// A builder for creating mock Solana accounts for testing purposes.
///
/// This struct provides a fluent API for configuring and building
/// Solana accounts with custom properties.
///
/// # Defaults
///
/// - **Owner**: System Program (`system_program::id()`) if not specified
/// - **Balance**: Rent-exempt amount based on data size if not explicitly set
/// - **Executable**: `false`
/// - **Rent Epoch**: `0`
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AccountBuilder {
    pubkey: Option<Pubkey>,
    lamports: Option<u64>,
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
        self.lamports = Some(lamports);
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

    /// Sets the account's pubkey.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use solana_program::pubkey::Pubkey;
    ///
    /// let pubkey = Pubkey::new_unique();
    /// let builder = AccountBuilder::new()
    ///     .pubkey(pubkey);
    /// ```
    pub fn pubkey(mut self, pubkey: Pubkey) -> Self {
        self.pubkey = Some(pubkey);
        self
    }

    /// Sets the account data using base64-encoded data.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    ///
    /// let base64_data = "SGVsbG8gV29ybGQ="; // "Hello World"
    /// let builder = AccountBuilder::new()
    ///     .data_base64(base64_data)
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if base64 decoding fails.
    pub fn data_base64(mut self, base64_data: &str) -> Result<Self, AccountGenError> {
        self.data = base64::decode(base64_data).map_err(|e| {
            AccountGenError::SerializationError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e,
            ))
        })?;
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
        self.try_build().expect("Failed to build account")
    }

    /// Builds the account and returns it with its pubkey.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use solana_program::pubkey::Pubkey;
    ///
    /// let pubkey = Pubkey::new_unique();
    /// let program_id = Pubkey::new_unique();
    /// let (account_pubkey, account) = AccountBuilder::new()
    ///     .pubkey(pubkey)
    ///     .balance(100_000_000)
    ///     .owner(program_id)
    ///     .build_with_pubkey();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the owner or pubkey has not been set.
    pub fn build_with_pubkey(self) -> (Pubkey, Account) {
        let pubkey = self.pubkey.expect("Account pubkey must be set");
        let account = self.build();
        (pubkey, account)
    }

    /// Attempts to build the account and return it with its pubkey.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use solana_program::pubkey::Pubkey;
    ///
    /// let pubkey = Pubkey::new_unique();
    /// let program_id = Pubkey::new_unique();
    /// let result = AccountBuilder::new()
    ///     .pubkey(pubkey)
    ///     .balance(100_000_000)
    ///     .owner(program_id)
    ///     .try_build_with_pubkey();
    /// assert!(result.is_ok());
    /// ```
    pub fn try_build_with_pubkey(self) -> Result<(Pubkey, Account), AccountGenError> {
        let pubkey = self.pubkey.ok_or(AccountGenError::MissingPubkey)?;
        let account = self.try_build()?;
        Ok((pubkey, account))
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

    /// Attempts to build the account, returning an error if required fields are missing.
    ///
    /// If no owner is specified, defaults to the System Program.
    /// If no balance is specified, defaults to rent-exempt amount for the data size.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use borsh::{BorshSerialize, BorshDeserialize};
    ///
    /// #[derive(BorshSerialize, BorshDeserialize)]
    /// struct MyData { value: u64 }
    ///
    /// // With defaults: owner = system program, balance = rent exempt
    /// let account = AccountBuilder::new()
    ///     .data(MyData { value: 42 })
    ///     .unwrap()
    ///     .try_build()
    ///     .unwrap();
    /// ```
    pub fn try_build(self) -> Result<Account, AccountGenError> {
        // Default to system program if owner not specified
        let owner = self.owner.unwrap_or_else(system_program::id);

        // Calculate rent-exempt balance if not specified
        let lamports = match self.lamports {
            Some(lamports) => lamports,
            None => {
                let rent = Rent::default();
                rent.minimum_balance(self.data.len())
            }
        };

        Ok(Account {
            lamports,
            data: self.data,
            owner,
            executable: self.executable,
            rent_epoch: self.rent_epoch,
        })
    }

    /// Creates an account with the given pubkey.
    ///
    /// # Example
    ///
    /// ```
    /// use solana_accountgen::AccountBuilder;
    /// use solana_program::pubkey::Pubkey;
    ///
    /// let pubkey = Pubkey::new_unique();
    /// let program_id = Pubkey::new_unique();
    /// let (account_pubkey, account) = AccountBuilder::new()
    ///     .balance(100_000)
    ///     .owner(program_id)
    ///     .create_with_pubkey(pubkey)
    ///     .unwrap();
    /// ```
    pub fn create_with_pubkey(self, pubkey: Pubkey) -> Result<(Pubkey, Account), AccountGenError> {
        let account = self.try_build()?;
        Ok((pubkey, account))
    }
}
