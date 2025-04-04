
# solana-accountgen

This crate provides a comprehensive set of tools for creating and managing mock Solana accounts in tests, making it much easier to write thorough tests for Solana programs.


## How It Works in Practice
When you're setting up a test for a Solana program, you typically:
- Create a ProgramTest instance
- Add your program and any accounts needed for testing
- Start the test environment
- Run your test transactions

With solana-accountgen, step 2 becomes much easier:

```rust       
// Create a program test environment
let program_id = Pubkey::new_unique();
let mut program_test = ProgramTest::new("my_program", program_id, None);

// Without solana-accountgen:
let mut account = Account::default();
account.lamports = 1_000_000;
account.owner = program_id;
// Manually serialize data...
account.data = serialize_data(my_data)?;
program_test.add_account(pubkey, account);

// With solana-accountgen:
program_test.add_account_with_builder(
    pubkey,
    AccountBuilder::new()
        .balance(1_000_000)
        .owner(program_id)
        .data(my_data)?
)?;
```

Here's a comprehensive list of what you can do with this crate:

## 1. Create Individual Test Accounts

```rust
// Create a basic account
let account = AccountBuilder::new()
    .balance(1_000_000)
    .owner(program_id)
    .build();

// Create an account with custom data
let account = AccountBuilder::new()
    .balance(1_000_000)
    .owner(program_id)
    .data(my_data_struct)?
    .build();

// Create an executable account (for program testing)
let program_account = AccountBuilder::new()
    .balance(1_000_000)
    .owner(bpf_loader::id())
    .executable(true)
    .data_raw(program_bytes)
    .build();
```

## 2. Create Accounts with Associated Pubkeys

```rust
// Create an account with its pubkey
let pubkey = Pubkey::new_unique();
let (account_pubkey, account) = AccountBuilder::new()
    .pubkey(pubkey)
    .balance(1_000_000)
    .owner(program_id)
    .build_with_pubkey();

// Create an account with a specific pubkey
let (account_pubkey, account) = create_account(
    pubkey,
    AccountBuilder::new()
        .balance(1_000_000)
        .owner(program_id)
).unwrap();
```

## 3. Manage Multiple Accounts with AccountMap

```rust
// Create an account map
let mut account_map = AccountMap::new();

// Add accounts to the map
account_map.add_with_builder(
    pubkey1,
    AccountBuilder::new()
        .balance(100_000)
        .owner(program_id)
).unwrap();

account_map.set_account(pubkey2, existing_account);

// Retrieve accounts
let account = account_map.get_account(&pubkey1).unwrap();

// Iterate over all accounts
for (pubkey, account) in account_map.iter() {
    println!("Account {} has {} lamports", pubkey, account.lamports);
}

// Create multiple accounts at once
let accounts = create_accounts(vec![
    (pubkey1, AccountBuilder::new().balance(100_000).owner(program_id)),
    (pubkey2, AccountBuilder::new().balance(200_000).owner(program_id)),
]).unwrap();
```

## 4. Create Program Derived Addresses (PDAs)

```rust
// Create a PDA with data
let seeds = &[b"game", player_pubkey.as_ref()];
let (pda, bump, account) = AccountBuilder::create_pda(
    &program_id,
    seeds,
    100_000,
    game_state
).unwrap();

// Verify the PDA matches expected address
assert_eq!(pda, expected_pda);
```

## 5. Integrate with solana-program-test

```rust
// Set up a program test environment
let mut program_test = ProgramTest::new("my_program", program_id, None);

// Add accounts using the extension trait
program_test.add_account_with_builder(
    pubkey,
    AccountBuilder::new()
        .balance(1_000_000)
        .owner(program_id)
        .data(my_data)?
).unwrap();

// Add multiple accounts at once
program_test.add_accounts(vec![
    (pubkey1, AccountBuilder::new().balance(100_000).owner(program_id)),
    (pubkey2, AccountBuilder::new().balance(200_000).owner(program_id)),
]).unwrap();

// Add all accounts from an AccountMap
program_test.add_account_map(account_map);
```

## 6. Create SPL Token Accounts

```rust
// Create a token account
let token_account = create_token_account(
    &mint_pubkey,
    &owner_pubkey,
    1000, // token amount
    &spl_token::id()
).unwrap();
```

## 7. Serialize and Deserialize Account Data

```rust
// Using Borsh serialization
let data = borsh_serialization::serialize_data(&my_data)?;
let deserialized = borsh_serialization::deserialize_account_data::<MyData>(&account)?;

// Using JSON serialization (via the "bincode" module)
let data = bincode_serialization::serialize_data(&my_data)?;
let deserialized = bincode_serialization::deserialize_account_data::<MyData>(&account)?;
```

## 8. Generate Accounts via CLI

```bash
# Generate a basic account in JSON format
solana-accountgen generate --balance 1000000 --owner 11111111111111111111111111111111

# Generate an account with data in base64 format
solana-accountgen generate --balance 1000000 --owner 11111111111111111111111111111111 --data 0102030405 --format base64
```

## 9. Test Account Serialization and Deserialization

```rust
// Create an account with serialized data
let account = AccountBuilder::new()
    .balance(100_000)
    .owner(program_id)
    .data(my_data)?
    .build();

// Deserialize the data for testing
let deserialized: MyData = borsh_serialization::deserialize_account_data(&account)?;
assert_eq!(deserialized.value, expected_value);
```

## 10. Create Accounts for End-to-End Testing

```rust
// Set up a complete test environment with multiple account types
let program_id = Pubkey::new_unique();
let user = Pubkey::new_unique();

// Create program account
let program_account = AccountBuilder::new()
    .balance(1_000_000)
    .owner(bpf_loader::id())
    .executable(true)
    .data_raw(program_bytes)
    .build();

// Create user state account
let user_state = UserState { balance: 100, active: true };
let user_account = AccountBuilder::new()
    .balance(10_000)
    .owner(program_id)
    .data(user_state)?
    .build();

// Create a PDA for the user
let (pda, _, pda_account) = AccountBuilder::create_pda(
    &program_id,
    &[b"user", user.as_ref()],
    10_000,
    UserPdaData { score: 42 }
)?;

// Add all accounts to a test environment
let mut program_test = ProgramTest::new("my_program", program_id, None);
program_test.add_account(program_id, program_account);
program_test.add_account(user, user_account);
program_test.add_account(pda, pda_account);
```