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
        let account_info = ctx.accounts.message_account.to_account_info();
        let data = &mut *account_info.try_borrow_mut_data().unwrap();

        let new_len = 8 + 4 + input.len();
        data[..new_len].copy_from_slice(input.as_bytes()); // Copy input directly without checking space

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

    #[account(mut)]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Message {
    pub message: String,
}
