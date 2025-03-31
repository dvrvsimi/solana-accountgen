use borsh::{BorshDeserialize, BorshSerialize};
use solana_accountgen::AccountBuilder;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MyData {
    value: u64,
    name: String,
}

fn main() {
    // Create a program ID
    let program_id = Pubkey::new_unique();
    println!("Program ID: {}", program_id);

    // Create account data
    let my_data = MyData {
        value: 42,
        name: "Test Account".to_string(),
    };

    // Build an account with the data
    let account = AccountBuilder::new()
        .balance(1_000_000) // 0.001 SOL
        .owner(program_id)
        .data(my_data)
        .unwrap()
        .build();

    println!("Account created with {} lamports", account.lamports);
    println!("Account data size: {} bytes", account.data.len());
    println!("Account owner: {}", account.owner);

    // Deserialize the data back
    let deserialized: MyData = MyData::try_from_slice(&account.data).unwrap();
    println!("Deserialized data: {:?}", deserialized);
} 