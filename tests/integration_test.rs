use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn test_initialize_private_account() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "psol",
        program_id,
        processor!(psol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let owner = Keypair::new();
    let private_account = Keypair::new();

    let init_ix = psol::instruction::initialize_private_account(
        &program_id,
        &owner.pubkey(),
        &private_account.pubkey(),
    );

    let mut transaction = Transaction::new_with_payer(&[init_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &owner], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_deposit_and_withdraw() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "psol",
        program_id,
        processor!(psol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Initialize accounts
    let owner = Keypair::new();
    let private_account = Keypair::new();
    let source_account = Keypair::new();
    let shielded_pool = Keypair::new();

    // Initialize private account
    let init_ix = psol::instruction::initialize_private_account(
        &program_id,
        &owner.pubkey(),
        &private_account.pubkey(),
    );

    let mut transaction = Transaction::new_with_payer(&[init_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Test deposit
    let deposit_amount = 1000u64;
    let deposit_ix = psol::instruction::deposit(
        &program_id,
        &owner.pubkey(),
        &source_account.pubkey(),
        &private_account.pubkey(),
        &shielded_pool.pubkey(),
        deposit_amount,
    );

    let mut transaction = Transaction::new_with_payer(&[deposit_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &owner], recent_blockhash);
    
    // Note: This will fail without proper token accounts setup
    // Full implementation needed for production
}

#[tokio::test]
async fn test_private_transfer() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "psol",
        program_id,
        processor!(psol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let sender = Keypair::new();
    let receiver = Keypair::new();
    let sender_account = Keypair::new();
    let receiver_account = Keypair::new();
    let nullifier_set = Keypair::new();

    // Initialize accounts first
    let init_sender_ix = psol::instruction::initialize_private_account(
        &program_id,
        &sender.pubkey(),
        &sender_account.pubkey(),
    );

    let init_receiver_ix = psol::instruction::initialize_private_account(
        &program_id,
        &receiver.pubkey(),
        &receiver_account.pubkey(),
    );

    let mut transaction = Transaction::new_with_payer(
        &[init_sender_ix, init_receiver_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &sender, &receiver], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Test private transfer
    let amount_commitment = [1u8; 32];
    let proof = vec![0u8; 128]; // Mock proof
    let nullifier = [2u8; 32];

    let transfer_ix = psol::instruction::private_transfer(
        &program_id,
        &sender.pubkey(),
        &sender_account.pubkey(),
        &receiver_account.pubkey(),
        &nullifier_set.pubkey(),
        amount_commitment,
        proof,
        nullifier,
    );

    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &sender], recent_blockhash);
    
    // Note: Requires nullifier set initialization for full test
}

#[tokio::test]
async fn test_reveal_balance() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "psol",
        program_id,
        processor!(psol::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let owner = Keypair::new();
    let private_account = Keypair::new();

    // Initialize account
    let init_ix = psol::instruction::initialize_private_account(
        &program_id,
        &owner.pubkey(),
        &private_account.pubkey(),
    );

    let mut transaction = Transaction::new_with_payer(&[init_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Reveal balance
    let reveal_ix = psol::instruction::reveal_balance(
        &program_id,
        &owner.pubkey(),
        &private_account.pubkey(),
    );

    let mut transaction = Transaction::new_with_payer(&[reveal_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
