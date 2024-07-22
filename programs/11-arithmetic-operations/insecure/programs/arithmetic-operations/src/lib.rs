use anchor_lang::prelude::*;

declare_id!("CCqc8QN2c33h6e17ykCnFJG4D1i9T9fR8hTXVvwy7qXp");

#[program]
pub mod arithmetic_operations {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.balance = amount;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.user_account;
        account.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.user_account;

        account.balance -= amount;
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub balance: u64,
}

#[derive(Accounts)]
#[instruction()]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + std::mem::size_of::<UserAccount>(),
        seeds = [b"user_account", initializer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
