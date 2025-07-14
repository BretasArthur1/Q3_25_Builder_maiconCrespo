use anchor_lang::prelude::*;
use anchor_vault_q3::instruction::{Close, Deposit, Initialize, Withdraw}; // Replace with your program's instructions
use litesvm_testing::prelude::*;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};


#[test]
fn test_escrow_operations() {
    // Initialize a LiteSVM environment
    let mut svm = LiteSVM::new();
    // Deploy the program to the SVM environment (replace with your program ID)
    let program_id = Pubkey::new_unique(); // Use your actual program ID in production
    svm.add_program("anchor_escrow", program_id);

    // Create a payer (simulates the user wallet)
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 5_000_000_000); // Airdrop 5 SOL for testing
