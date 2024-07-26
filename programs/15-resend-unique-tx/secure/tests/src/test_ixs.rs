use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster, Program,
};
use resend::{accounts, instruction};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

fn setup_program() -> (Client<Arc<Keypair>>, Program<Arc<Keypair>>, Keypair) {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Arc::new(read_keypair_file(&anchor_wallet).unwrap());
    let client = Client::new_with_options(
        Cluster::Localnet,
        Arc::clone(&payer),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(resend::id()).unwrap();

    (client, program, payer.insecure_clone())
}

#[test]
fn test_resend() {
    let (_client, program, authority) = setup_program();

    let (data_account, _bump) = Pubkey::find_program_address(
        &[b"data_account", authority.pubkey().as_ref()],
        &resend::id(),
    );

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let tx_id = format!("{}{}", timestamp, authority.pubkey());

    let _tx = program
        .request()
        .accounts(accounts::Transfer {
            data_account: data_account,
            initializer: authority.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::Transfer {
            amount: 0,
            tx_id: tx_id.clone(),
        })
        .send()
        .expect("Failed to send transfer transaction");

    let _replay_tx = program
        .request()
        .accounts(accounts::Transfer {
            data_account: data_account,
            initializer: authority.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::Transfer {
            amount: 0,
            tx_id: tx_id, // Pass the same hash
        })
        .send()
        .expect("Failed to send replay transaction");
}
