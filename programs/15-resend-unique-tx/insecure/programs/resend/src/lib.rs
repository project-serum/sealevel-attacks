use anchor_lang::prelude::*;

declare_id!("wRac42enipN1hfJgkgqtuXbprsX9p6yNEZB6zypErzC");

#[program]
pub mod resend {
    use super::*;

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.data_account;
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
        space = 64 * 64,
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
}
