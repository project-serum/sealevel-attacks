use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod owner_checks_recommended {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        // Log the token amount directly from the TokenAccount
        msg!("Your account balance is: {}", ctx.accounts.token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    // Token account information with a constraint to ensure the authority is the owner
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    // Authority that must sign the transaction
    authority: Signer<'info>,
}
