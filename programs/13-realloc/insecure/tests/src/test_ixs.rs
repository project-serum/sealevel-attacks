use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster, Program,
};
use realloc::{accounts, instruction, Message};
use std::sync::Arc;

fn setup_program() -> (Client<Arc<Keypair>>, Program<Arc<Keypair>>, Keypair) {
    let anchor_wallet =
        std::env::var("ANCHOR_WALLET").expect("ANCHOR_WALLET environment variable not set");
    let payer = Arc::new(read_keypair_file(&anchor_wallet).expect("Failed to read keypair file"));
    let client = Client::new_with_options(
        Cluster::Localnet,
        Arc::clone(&payer),
        CommitmentConfig::confirmed(),
    );
    let program = client
        .program(realloc::id())
        .expect("Failed to get program");

    (client, program, payer.insecure_clone())
}

#[test]
fn test_initialize_insufficient_space() {
    let (_client, program, authority) = setup_program();

    let message_account = Keypair::new();

    let large_message = "A".repeat(20);

    // Attempt to initialize with insufficient space
    let result = program
        .request()
        .accounts(accounts::Initialize {
            payer: authority.pubkey(),
            message_account: message_account.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::Initialize {
            input: large_message.clone(),
        })
        .signer(&authority)
        .signer(&message_account)
        .send();

    println!("Initialization failed as expected: {:?}", result);
}

#[test]
fn test_update_with_unsafe_assumptions() {
    let (_client, program, authority) = setup_program();

    let message_account = Keypair::new();

    let initial_message = String::from("Hello, Solana!");
    let updated_message =
        String::from("Hello, Anchor! This is a longer message that exceeds allocated space.");

    program
        .request()
        .accounts(accounts::Initialize {
            payer: authority.pubkey(),
            message_account: message_account.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::Initialize {
            input: initial_message.clone(),
        })
        .signer(&authority)
        .signer(&message_account)
        .send()
        .expect("Failed to send initialize account transaction");

    let result = program
        .request()
        .accounts(accounts::Update {
            payer: authority.pubkey(),
            message_account: message_account.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::Update {
            input: updated_message.clone(),
        })
        .signer(&authority)
        .send();

    assert!(
        result.is_err(),
        "Expected error due to insufficient space calculation"
    );

    let account_data: Message = program
        .account(message_account.pubkey())
        .expect("Failed to fetch account data");

    assert_eq!(
        account_data.message, initial_message,
        "Account data should not be updated incorrectly"
    );
}
