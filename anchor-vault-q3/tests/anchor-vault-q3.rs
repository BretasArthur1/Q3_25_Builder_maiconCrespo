use anchor_lang::prelude::*;
use anchor_vault_q3::instruction::{Close, Deposit, Initialize, Withdraw}; // Replace with your program's instructions
use litesvm_testing::prelude::*;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[test]
fn test_vault_operations() {
    // Initialize a LiteSVM environment
    let mut svm = LiteSVM::new();

    // Deploy the program to the SVM environment (replace with your program ID)
    let program_id = Pubkey::new_unique(); // Use your actual program ID in production
    svm.add_program("anchor_vault_q3", program_id);

    // Create a payer (simulates the user wallet)
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000_000); // Airdrop 10 SOL for testing

    // Derive PDAs for vaultState and vault (equivalent to findProgramAddressSync in JS)
    let vault_state =
        Pubkey::find_program_address(&[b"state", payer.pubkey().as_ref()], &program_id).0;
    let vault = Pubkey::find_program_address(&[b"vault", vault_state.as_ref()], &program_id).0;

    // Test 1: Initialize
    let initialize_ix = Initialize {}.into_instruction(&program_id);
    svm.process_instruction(&initialize_ix, &[&payer], &[vault_state, vault])
        .expect("Initialize failed");
    println!("Vault initialized");

    // Test 2: Deposit 2 SOL
    let deposit_amount = 2_000_000_000; // 2 SOL in lamports
    let deposit_ix = Deposit {
        amount: deposit_amount,
    }
    .into_instruction(&program_id);
    svm.process_instruction(&deposit_ix, &[&payer], &[vault_state, vault])
        .expect("Deposit failed");
    println!(
        "Deposited 2 SOL, Vault balance: {}",
        svm.get_balance(&vault)
    );

    // Test 3: Withdraw 1 SOL
    let withdraw_amount = 1_000_000_000; // 1 SOL in lamports
    let withdraw_ix = Withdraw {
        amount: withdraw_amount,
    }
    .into_instruction(&program_id);
    svm.process_instruction(&withdraw_ix, &[&payer], &[vault_state, vault])
        .expect("Withdraw failed");
    println!(
        "Withdrawn 1 SOL, Vault balance: {}",
        svm.get_balance(&vault)
    );

    // Test 4: Close Vault
    let close_ix = Close {}.into_instruction(&program_id);
    svm.process_instruction(&close_ix, &[&payer], &[vault_state, vault])
        .expect("Close failed");
    println!(
        "Vault closed, Vault account info: {:?}",
        svm.get_account(&vault)
    );
}
