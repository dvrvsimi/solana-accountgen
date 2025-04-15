//! Command-line interface for solana-accountgen
//!
//! This binary provides a CLI for generating mock Solana accounts
//! for testing purposes. It allows users to create accounts with
//! specific properties and output them in various formats.

use base64;
use clap::{Parser, Subcommand};
use hex;
use serde_json;
use solana_accountgen::AccountBuilder;
use solana_pubkey::Pubkey;
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

        /// Account data as a hex string (e.g., "0102ABCD")
        #[arg(short, long)]
        data: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            balance,
            owner,
            executable,
            format,
            data,
        } => {
            let owner_pubkey = Pubkey::from_str(&owner)?;

            // Start building the account
            let mut builder = AccountBuilder::new()
                .balance(balance)
                .owner(owner_pubkey)
                .executable(executable);

            // Add data if provided
            if let Some(hex_data) = data {
                // Parse hex string to bytes
                let data_bytes = hex::decode(&hex_data)?;
                builder = builder.data_raw(data_bytes);
            }

            // Build the account
            let account = builder.build();

            match format.as_str() {
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&account)?);
                }
                "base64" => {
                    // Serialize using serde_json instead of bincode
                    let json_bytes = serde_json::to_vec(&account)?;

                    // Encode as base64
                    let base64_string = base64::encode(&json_bytes);

                    // Print the result
                    println!("{}", base64_string);
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
