use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod duplicate_mutable_accounts_recommended {
    use super::*;

    pub fn update(ctx: Context<Update>, a: u64, b: u64) -> ProgramResult {
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        // Updating the data fields of both accounts
        user_a.data = a;
        user_b.data = b;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Update<'info> {
    // Constraint ensures that user_a and user_b are different accounts
    #[account(constraint = user_a.key() != user_b.key())]
    user_a: Account<'info, User>,
    user_b: Account<'info, User>,
}

#[account]
pub struct User {
    data: u64,
}
