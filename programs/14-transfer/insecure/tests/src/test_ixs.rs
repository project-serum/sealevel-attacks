use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster, Program,
};
use std::sync::Arc;
use transfer::{accounts, instruction};

fn setup_program() -> (Client<Arc<Keypair>>, Program<Arc<Keypair>>, Arc<Keypair>) {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = Arc::new(read_keypair_file(&anchor_wallet).unwrap());
    let client = Client::new_with_options(
        Cluster::Localnet,
        Arc::clone(&payer),
        CommitmentConfig::confirmed(),
    );
    let program = client.program(transfer::id()).unwrap();

    (client, program, Arc::clone(&payer))
}

#[test]
fn test_transfer_tokens_with_malicious_address() {
    let (_client, program, authority) = setup_program();

    let key_slice: [u8; 2] = [0x41, 0x41];

    let mut pubkey_slice = [0u8; 32];

    pubkey_slice[..2].copy_from_slice(&key_slice);
    let keypair1 = Keypair::new();
    let keypair2 = Keypair::new();

    let malicious_pubkey = Pubkey::new_from_array(pubkey_slice);

    let _add_tx = program
        .request()
        .accounts(accounts::AddBalance {
            authority: authority.pubkey(),
            user_account: keypair1.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::AddBalance {
            pubkey: keypair1.pubkey(),
            amount: 1000,
        })
        .signer(&authority)
        .signer(&keypair1)
        .send();

    let _add_tx = program
        .request()
        .accounts(accounts::AddBalance {
            authority: authority.pubkey(),
            user_account: keypair2.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::AddBalance {
            pubkey: malicious_pubkey,
            amount: 1000,
        })
        .signer(&authority)
        .signer(&keypair2)
        .send();

    let transfer_tx = program
        .request()
        .accounts(accounts::Transfer {
            authority: authority.pubkey(),
            from: keypair1.pubkey(),
            to: keypair2.pubkey(),
            system_program: system_program::id(),
        })
        .args(instruction::Transfer {
            amount: 50,
            to: malicious_pubkey,
        })
        .signer(&keypair1)
        .signer(&keypair2)
        .send();
}
