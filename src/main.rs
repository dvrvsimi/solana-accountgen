//! Command-line interface for solana-accountgen
//!
//! This binary provides a CLI for generating mock Solana accounts
//! for testing purposes. It allows users to create accounts with
//! specific properties and output them in various formats.

use clap::{Parser, Subcommand};
use solana_accountgen::AccountBuilder;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

/// CLI for generating Solana test accounts
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    /// Generate a new account
    Generate {
        /// Account balance in lamports
        #[arg(short, long, default_value = "0")]
        balance: u64,

        /// Account owner (as base58 encoded public key)
        #[arg(short, long)]
        owner: String,

        /// Whether the account is executable
        #[arg(short, long, default_value = "false")]
        executable: bool,

        /// Output format (json or base64)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { balance, owner, executable, format } => {
            let owner_pubkey = Pubkey::from_str(&owner)?;
            
            let account = AccountBuilder::new()
                .balance(balance)
                .owner(owner_pubkey)
                .executable(executable)
                .build();
            
            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&account)?);
                }
                "base64" => {
                    // Simplified for example purposes
                    println!("Account data would be base64 encoded here");
                }
                _ => {
                    eprintln!("Unsupported format: {}", format);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
