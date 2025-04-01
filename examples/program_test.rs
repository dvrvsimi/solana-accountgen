use borsh::{BorshDeserialize, BorshSerialize};
use solana_accountgen::AccountBuilder;
use solana_program::pubkey::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::signature::Signer;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CounterData {
    count: u64,
}

#[tokio::main]
async fn main() {
    // Create a program test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "my_program",
        program_id,
        None,
    );

    // Create an account using AccountBuilder
    let account_pubkey = Pubkey::new_unique();
    let account_builder = AccountBuilder::new()
        .balance(1_000_000)
        .owner(program_id);

    // Add the account to the test environment
    program_test.add_account(
        account_pubkey,
        account_builder.build(),
    );

    // Start the test environment
    let (_banks_client, payer, _recent_blockhash) = program_test.start().await;

    // Now you can use the test environment to test your program
    println!("Test environment started with payer: {}", payer.pubkey());
} 