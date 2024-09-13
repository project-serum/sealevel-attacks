use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod closing_accounts_recommended {
    use super::*;

    pub fn close(ctx: Context<Close>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    // Automatically transfer lamports from the account to the destination and close the account
    #[account(mut, close = destination)]
    account: Account<'info, Data>,
    #[account(mut)]
    destination: AccountInfo<'info>, // Receiver of the remaining lamports
}

#[account]
pub struct Data {
    data: u64,
}
