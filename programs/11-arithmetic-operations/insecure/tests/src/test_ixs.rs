use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster, Program,
};
use arithmetic_operations::{accounts, instruction};
use std::sync::Arc;

fn setup_program() -> (Client<Arc<Keypair>>, Program<Arc<Keypair>>, Keypair) {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Arc::new(read_keypair_file(&anchor_wallet).unwrap());
    let client = Client::new_with_options(
        Cluster::Localnet,
        Arc::clone(&payer),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(arithmetic_operations::id()).unwrap();

    (client, program, payer.insecure_clone())
}

#[test]
fn test_deposit() {
    let (_client, program, authority) = setup_program();

    let (user_account, _bump) = Pubkey::find_program_address(
        &[b"user_account", authority.pubkey().as_ref()],
        &arithmetic_operations::id(),
    );

    let _init = program
        .request()
        .accounts(accounts::Initialize {
            initializer: authority.pubkey(),
            user_account: user_account,
            system_program: system_program::id(),
        })
        .args(instruction::Initialize { amount: 10 })
        .send()
        .expect("Failed to send initialize account transaction");

    let tx = program
        .request()
        .accounts(accounts::Deposit {
            user_account: user_account,
        })
        .args(instruction::Deposit { amount: u64::MAX })
        .send()
        .expect("Failed to send initialize account transaction");

    println!("Initialize transaction signature: {}", tx);
}
