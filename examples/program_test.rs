use borsh::{BorshDeserialize, BorshSerialize};
use solana_accountgen::AccountBuilder;
use solana_program_test::ProgramTest;
use solana_pubkey::Pubkey;
use solana_signer::Signer;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CounterData {
    count: u64,
}

#[tokio::main]
async fn main() {
    // Create a program test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new("my_program", program_id, None);

    // Create an account using AccountBuilder with pubkey
    let account_pubkey = Pubkey::new_unique();
    let (_, account) = AccountBuilder::new()
        .pubkey(account_pubkey)
        .balance(1_000_000)
        .owner(program_id)
        .build_with_pubkey();

    // Add the account to the test environment
    program_test.add_account(account_pubkey, account);

    // Start the test environment
    let (_banks_client, payer, _recent_blockhash) = program_test.start().await;

    // Now you can use the test environment to test your program
    println!("Test environment started with payer: {}", payer.pubkey());
}
