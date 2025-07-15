use litesvm::LiteSVM;
use spl_associated_token_account::get_associated_token_address;
use spl_token_2022::instruction::initialize_mint2;

// Program ID  ( declare_id!  lib.rs)
const PROGRAM_ID: &str = "ADxxvPiPKT1SeQLDzukVxRwLggTXRXVvUR3eSxLRS6Yi";

//Auxiliar function to create an mint
// Función auxiliar para crear un mint
async fn create_mint(
    svm: &mut litesvm::ProgramTestContext,
    mint_keypair: &Keypair,
    authority: &solana_sdk::pubkey::Pubkey,
) {
    let rent = svm.banks_client.get_rent().await.unwrap();
    let mint_size = spl_token_2022::state::Mint::LEN;
    let lamports = rent.minimum_balance(mint_size);

    let create_account_ix = solana_sdk::system_instruction::create_account(
        &svm.payer.pubkey(),
        &mint_keypair.pubkey(),
        lamports,
        mint_size as u64,
        &spl_token_2022::ID,
    );

    let init_mint_ix = initialize_mint2(
        &spl_token_2022::ID,
        &mint_keypair.pubkey(),
        authority,
        None,
        9, // decimals
    )
    .unwrap();

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_account_ix, init_mint_ix],
        Some(&svm.payer.pubkey()),
        &[&svm.payer, mint_keypair],
        svm.last_blockhash,
    );

    svm.send_transaction(tx).await.unwrap();
}

// Auxiliar function to create an ATA
// Función auxiliar para crear una cuenta de token asociada
async fn create_associated_token_account(
    svm: &mut litesvm::ProgramTestContext,
    owner: &solana_sdk::pubkey::Pubkey,
    mint: &solana_sdk::pubkey::Pubkey,
) {
    let ata = get_associated_token_address(owner, mint);
    let create_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &svm.payer.pubkey(),
        owner,
        mint,
        &spl_token_2022::ID,
    );
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_ix],
        Some(&svm.payer.pubkey()),
        &[&svm.payer],
        svm.last_blockhash,
    );
    svm.send_transaction(tx).await.unwrap();
}
//Auxiliar Function to mint an account
// Función auxiliar para hacer mint a una cuenta
async fn mint_to(
    svm: &mut litesvm::ProgramTestContext,
    mint: &solana_sdk::pubkey::Pubkey,
    to: &solana_sdk::pubkey::Pubkey,
    amount: u64,
) {
    let ata = get_associated_token_address(to, mint);

    let mint_ix = spl_token_2022::instruction::mint_to(
        &spl_token_2022::ID,
        mint,
        &ata,
        &svm.payer.pubkey(),
        &[],
        amount,
    )
    .unwrap();

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&svm.payer.pubkey()),
        &[&svm.payer],
        svm.last_blockhash,
    );

    svm.send_transaction(tx).await.unwrap();
}

#[tokio::test]
async fn test_svm_startup() {
    // Inicia LiteSVM con el programa (aún sin tu programa cargado)
    let mut svm = program_test!().start_with_context().await;

    // Crea una nueva cuenta para probar
    let account = Keypair::new();

    // Envía SOL (en lamports) a la cuenta
    svm.airdrop(&account.pubkey(), 1_000_000_000)
        .await
        .unwrap();

    // Obtiene el balance de la cuenta
    let balance = svm.get_balance(&account.pubkey()).await;

    // Verifica que se haya realizado el airdrop
    assert_eq!(balance, 1_000_000_000);
}

#[tokio::test]
async fn test_setup_mints_and_accounts() {
    // Inicia el entorno LiteSVM y carga tu programa
    let mut svm = program_test!(PROGRAM_ID)
        .start_with_context()
        .await;

    // Crea claves para mint A y B
    let mint_a = Keypair::new();
    let mint_b = Keypair::new();

    // Crea cuentas para maker y taker
    let maker = Keypair::new();
    let taker = Keypair::new();

    // Airdrop SOL a maker y taker
    svm.airdrop(&maker.pubkey(), 1_000_000_000).await.unwrap();
    svm.airdrop(&taker.pubkey(), 1_000_000_000).await.unwrap();

    // Crear mint A y B
    create_mint(&mut svm, &mint_a, &maker.pubkey()).await;
    create_mint(&mut svm, &mint_b, &taker.pubkey()).await;

    // Crear cuentas de token asociadas
    create_associated_token_account(&mut svm, &maker.pubkey(), &mint_a.pubkey()).await;
    create_associated_token_account(&mut svm, &taker.pubkey(), &mint_b.pubkey()).await;

    // Hacer mint de tokens
    mint_to(&mut svm, &mint_a.pubkey(), &maker.pubkey(), 1_000_000).await;
    mint_to(&mut svm, &mint_b.pubkey(), &taker.pubkey(), 1_000_000).await;

    // Verificar balances
    let ata_maker_a = get_associated_token_address(&maker.pubkey(), &mint_a.pubkey());
    let ata_taker_b = get_associated_token_address(&taker.pubkey(), &mint_b.pubkey());

    let balance_maker_a = svm.get_token_balance(&ata_maker_a).await;
    let balance_taker_b = svm.get_token_balance(&ata_taker_b).await;

    assert_eq!(balance_maker_a, 1_000_000);
    assert_eq!(balance_taker_b, 1_000_000);
}
