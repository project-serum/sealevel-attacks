use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("4XN8hDmUAuasUdaSguFTHRkT34j8gcwXKg9o55qv2Pjy");

#[program]
pub mod reinitialization {
    use super::*;

    pub fn init_counter(ctx: Context<InitCounter>, authority: Pubkey) -> Result<()> {
        let account = &mut ctx.accounts.counter_account;
        if account.initialized {
            return Err(ProgramError::AccountAlreadyInitialized.into());
        }
        account.count = 0;
        account.authority = authority;
        account.initialized = true;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let account = &mut ctx.accounts.counter_account;
        account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCounter<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = size_of::<CounterAccount>() + 8,
        seeds = [b"counter-seed"],
        bump
    )]
    pub counter_account: Account<'info, CounterAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter_account: Account<'info, CounterAccount>,

    pub authority: Signer<'info>,
}

#[account]
pub struct CounterAccount {
    pub count: u64,
    pub authority: Pubkey,
    pub initialized: bool,
}
