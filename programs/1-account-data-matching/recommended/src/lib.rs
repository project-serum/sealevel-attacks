use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod account_data_matching_recommended {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        // Logging the account balance directly from the validated token account
        msg!("Your account balance is: {}", ctx.accounts.token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    // Validating that the authority is the owner of the token account
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    // Authority must sign the transaction and own the token account
    authority: Signer<'info>,
}
