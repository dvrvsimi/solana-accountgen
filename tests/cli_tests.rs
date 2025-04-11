use assert_cmd::Command;
use base64;
#[allow(unused_imports)]
use predicates::prelude::*;
use serde_json;
use solana_account::Account;
use solana_pubkey::Pubkey;
use std::str::FromStr;

#[test]
fn test_cli_base64_output() {
    // Create a known public key for testing
    let owner = "11111111111111111111111111111111";
    let owner_pubkey = Pubkey::from_str(owner).unwrap();

    // Run the CLI command
    let mut cmd = Command::cargo_bin("solana-accountgen").unwrap();
    let output = cmd
        .arg("generate")
        .arg("--balance")
        .arg("1000000")
        .arg("--owner")
        .arg(owner)
        .arg("--format")
        .arg("base64")
        .output()
        .expect("Failed to execute command");

    // Get the output as a string
    let base64_output = String::from_utf8(output.stdout).unwrap().trim().to_string();

    // Decode the base64 output
    let decoded_bytes = base64::decode(&base64_output).unwrap();

    // Deserialize with serde_json
    let decoded_account: Account = serde_json::from_slice(&decoded_bytes).unwrap();

    // Verify the account properties
    assert_eq!(decoded_account.lamports, 1000000);
    assert_eq!(decoded_account.owner, owner_pubkey);
    assert_eq!(decoded_account.executable, false);
}

#[test]
fn test_cli_base64_output_with_data() {
    // known public key for testing
    let owner = "11111111111111111111111111111111";
    let owner_pubkey = Pubkey::from_str(owner).unwrap();

    // Run the CLI command with data
    let mut cmd = Command::cargo_bin("solana-accountgen").unwrap();
    let output = cmd
        .arg("generate")
        .arg("--balance")
        .arg("1000000")
        .arg("--owner")
        .arg(owner)
        .arg("--data")
        .arg("0102030405") // hex data
        .arg("--format")
        .arg("base64")
        .output()
        .expect("Failed to execute command");

    // Get the output as a string
    let base64_output = String::from_utf8(output.stdout).unwrap().trim().to_string();

    // Decode the base64 output
    let decoded_bytes = base64::decode(&base64_output).unwrap();

    // Deserialize with serde_json
    let decoded_account: Account = serde_json::from_slice(&decoded_bytes).unwrap();

    // Verify the account properties
    assert_eq!(decoded_account.lamports, 1000000);
    assert_eq!(decoded_account.owner, owner_pubkey);
    assert_eq!(decoded_account.executable, false);
    assert_eq!(decoded_account.data, vec![1, 2, 3, 4, 5]); // check data
}
