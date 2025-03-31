use borsh::{BorshDeserialize, BorshSerialize};
use solana_accountgen::{AccountBuilder, extensions::program_test::ProgramTestExt};
use solana_program::{pubkey::Pubkey, system_program};
use solana_program_test::ProgramTest;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CounterData {
    count: u64,
}

#[tokio::main]
async fn main() {
    // Create a program ID and a program test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "counter_program", // This would be your program name
        program_id,
        None, // processor
    );

    // Create a counter account
    let counter_keypair = Keypair::new();
    let counter_pubkey = counter_keypair.pubkey();
    
    let counter_data = CounterData { count: 0 };
    
    // Add the counter account to the test environment
    program_test.add_account_with_builder(
        counter_pubkey,
        AccountBuilder::new()
            .balance(1_000_000)
            .owner(program_id)
            .data(counter_data)
            .unwrap(),
    ).unwrap();

    // Start the test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
    println!("Test environment started!");
    println!("Counter account: {}", counter_pubkey);
    println!("Payer: {}", payer.pubkey());
    
    // In a real test, you would now interact with your program
    // using the banks_client
} 