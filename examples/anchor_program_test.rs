//! Example of testing an Anchor program using solana-accountgen.
//!
//! This example demonstrates how to:
//! - Set up a test environment for an Anchor program
//! - Create accounts with Anchor's discriminator
//! - Create and send transactions with Anchor's method discriminator
//! - Verify account state after transaction execution
//!
//! # How Anchor Integration Works
//!
//! Anchor programs use discriminators to identify account types and instruction methods:
//!
//! 1. **Account Discriminators**: Each account type has an 8-byte discriminator at the
//!    beginning of its data. This is the first 8 bytes of the SHA-256 hash of
//!    "account:{account_type}".
//!
//! 2. **Instruction Discriminators**: Each instruction method has an 8-byte discriminator
//!    at the beginning of its data. This is the first 8 bytes of the SHA-256 hash of
//!    "global:{method_name}".
//!
//! The solana-accountgen library provides helpers to create accounts and instructions
//! with these discriminators automatically, making it easier to test Anchor programs.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_accountgen::{
    AccountBuilder, 
    extensions::
        anchor::{create_anchor_account, create_anchor_instruction, deserialize_anchor_account},
};
use solana_program::{
    pubkey::Pubkey, 
    system_program,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::AccountMeta,
};

// Define the account structure that our Anchor program would use
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct GameAccount {
    player: Pubkey,
    score: u64,
    is_initialized: bool,
}

// Define the instruction data structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum GameInstruction {
    Initialize,
    UpdateScore { new_score: u64 },
}

#[tokio::main]
async fn main() {
    // Set up the program test environment
    let program_id = Pubkey::new_unique(); // In a real test, use your actual program ID
    let mut program_test = ProgramTest::new(
        "my_anchor_program",
        program_id,
        processor!(process_instruction), // Your program's entrypoint
    );
    
    // Create test accounts
    let player = Keypair::new();
    let game_account_keypair = Keypair::new();
    
    // Fund the player account
    program_test.add_account(
        player.pubkey(),
        AccountBuilder::new()
            .balance(1_000_000_000) // 1 SOL
            .owner(system_program::id())
            .build()
    );
    
    // Create the game state
    let game_state = GameAccount {
        player: player.pubkey(),
        score: 0,
        is_initialized: true,
    };
    
    // Add the game account with Anchor's discriminator
    // There are two ways to do this:
    
    // Method 1: Using create_anchor_account directly
    program_test.add_account(
        game_account_keypair.pubkey(),
        create_anchor_account(
            "game_account", // The account type name in your Anchor program
            program_id,
            game_state.clone(),
            10_000_000, // Rent exempt amount
        ).unwrap()
    );
    
    // Method 2: Using the ProgramTestExt trait (commented out to avoid duplicate account)
    /*
    program_test.add_anchor_account(
        game_account_keypair.pubkey(),
        "game_account",
        program_id,
        game_state,
        10_000_000,
    ).unwrap();
    */
    
    // Start the test
    let (banks_client, payer, recent_blockhash) = program_test.start().await;
    
    // Create an instruction to update the score
    // This automatically adds the Anchor method discriminator
    let update_score_ix = create_anchor_instruction(
        program_id,
        "update_score", // The method name in your Anchor program
        vec![
            AccountMeta::new(game_account_keypair.pubkey(), false),
            AccountMeta::new_readonly(player.pubkey(), true),
        ],
        GameInstruction::UpdateScore { new_score: 100 },
    ).unwrap();
    
    // Create and send the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[update_score_ix],
        Some(&payer.pubkey()),
        &[&payer, &player],
        recent_blockhash,
    );
    
    banks_client.process_transaction(transaction).await.unwrap();
    
    // Verify the game state was updated
    // This automatically skips the Anchor discriminator when deserializing
    let game_account = banks_client.get_account(game_account_keypair.pubkey()).await.unwrap().unwrap();
    let updated_game_state: GameAccount = deserialize_anchor_account(&game_account).unwrap();
    
    assert_eq!(updated_game_state.score, 100);
    assert_eq!(updated_game_state.player, player.pubkey());
    assert_eq!(updated_game_state.is_initialized, true);
    
    println!("Test passed! Game state updated successfully.");
}

// Mock processor function (in a real test, this would be your actual program)
// This simulates how an Anchor program would process instructions
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // In a real test, this would call your program's processor
    // For this example, we'll just simulate updating the score
    
    // Skip the 8-byte discriminator
    let instruction = borsh::from_slice::<GameInstruction>(&instruction_data[8..]).unwrap();
    
    match instruction {
        GameInstruction::UpdateScore { new_score } => {
            let game_account_info = &accounts[0];
            let mut game_data = game_account_info.try_borrow_mut_data().unwrap();
            
            // Skip the 8-byte discriminator
            let mut game_state = GameAccount::try_from_slice(&game_data[8..]).unwrap();
            game_state.score = new_score;
            
            // Write back the updated state (preserving the discriminator)
            let serialized = borsh::to_vec(&game_state).unwrap();
            game_data[8..8+serialized.len()].copy_from_slice(&serialized);
        }
        _ => {}
    }
    
    Ok(())
} 