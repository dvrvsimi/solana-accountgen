use borsh::{BorshDeserialize, BorshSerialize};
use solana_accountgen::extensions::anchor::{
    create_anchor_account, create_anchor_instruction, deserialize_anchor_account,
    get_account_discriminator, get_method_discriminator,
};
use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
struct TestAccount {
    value: u64,
}

#[test]
fn test_anchor_account_discriminator() {
    let discriminator = get_account_discriminator("test_account");
    assert_eq!(discriminator.len(), 8);
}

#[test]
fn test_anchor_method_discriminator() {
    let discriminator = get_method_discriminator("test_method");
    assert_eq!(discriminator.len(), 8);
}

#[test]
fn test_create_anchor_account() {
    let program_id = Pubkey::new_unique();
    let test_data = TestAccount { value: 42 };

    let account =
        create_anchor_account("test_account", program_id, test_data.clone(), 1_000_000).unwrap();

    // Verify account properties
    assert_eq!(account.owner, program_id);
    assert_eq!(account.lamports, 1_000_000);

    // Verify data with discriminator
    assert!(account.data.len() > 8);

    // Deserialize and verify
    let deserialized: TestAccount = deserialize_anchor_account(&account).unwrap();
    assert_eq!(deserialized, test_data);
}

#[test]
fn test_create_anchor_instruction() {
    let program_id = Pubkey::new_unique();
    let test_data = TestAccount { value: 42 };
    let accounts = vec![AccountMeta::new(Pubkey::new_unique(), false)];

    let ix =
        create_anchor_instruction(program_id, "test_method", accounts.clone(), test_data).unwrap();

    // Verify instruction properties
    assert_eq!(ix.program_id, program_id);
    assert_eq!(ix.accounts, accounts);
    assert!(ix.data.len() > 8);
}
