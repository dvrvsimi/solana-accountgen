use borsh::{BorshDeserialize, BorshSerialize};
use solana_accountgen::AccountBuilder;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct GameState {
    player: Pubkey,
    score: u64,
    level: u8,
}

fn main() {
    // Create a program ID
    let program_id = Pubkey::new_unique();
    println!("Program ID: {}", program_id);

    // Create a player pubkey
    let player = Pubkey::new_unique();
    println!("Player: {}", player);

    // Find a PDA for the game state
    let seeds = &[b"game", player.as_ref()];
    let (pda, bump) = Pubkey::find_program_address(seeds, &program_id);
    println!("PDA: {} (bump: {})", pda, bump);

    // Create game state data
    let game_state = GameState {
        player,
        score: 1000,
        level: 5,
    };

    // Build an account for the PDA
    let account = AccountBuilder::new()
        .balance(10_000_000) // Rent exempt amount
        .owner(program_id)
        .data(game_state)
        .unwrap()
        .build();

    println!("PDA account created with {} lamports", account.lamports);
    println!("PDA data size: {} bytes", account.data.len());

    // Deserialize the data back
    let deserialized: GameState = GameState::try_from_slice(&account.data).unwrap();
    println!("Deserialized game state: {:?}", deserialized);
} 