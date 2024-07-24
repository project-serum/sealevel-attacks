use anchor_lang::prelude::*;

declare_id!("CaxrUZLmgDYiZAeAzBwCHrsM9QyqNyAvWMqpu7KFWJa");

#[program]
pub mod realloc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 4 + input.len(),
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Update<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        realloc = 8 + 4 + input.len(),
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Message {
    pub message: String,
}
