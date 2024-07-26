use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::hash;

declare_id!("wRac42enipN1hfJgkgqtuXbprsX9p6yNEZB6zypErzC");

const MAX_PROCESSED_TXS: usize = 32;

#[program]
pub mod resend {
    use super::*;

    pub fn transfer(ctx: Context<Transfer>, tx_id: String, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.data_account;
        let tx_hash = hash(tx_id.as_bytes());
        let tx_hash_arr: [u8; 32] = tx_hash.to_bytes();

        if account.processed_txs.contains(&tx_hash_arr) {
            return Err(ErrorMsg::TransactionAlreadyProcessed.into());
        }

        if account.processed_txs.len() >= MAX_PROCESSED_TXS {
            account.processed_txs.remove(0); // Remove oldest hash
        }
        account.processed_txs.push(tx_hash_arr);

        account.balance = account
            .balance
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        init_if_needed,
        payer = initializer,
        space = 8 + 8 + 8 + (4 + 32 * 10),
        seeds = [b"data_account", initializer.key().as_ref()],
        bump
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DataAccount {
    pub balance: u64,
    pub processed_txs: Vec<[u8; MAX_PROCESSED_TXS]>,
}

#[error_code]
pub enum ErrorMsg {
    #[msg("Transaction hash has already been processed.")]
    TransactionAlreadyProcessed,
}
