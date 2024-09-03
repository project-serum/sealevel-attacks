use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod account_data_matching_insecure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        // Directly unpacking token account data without validating ownership
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        // Logging the account balance
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    // This account is assumed to be a valid token account without verification
    token: AccountInfo<'info>,
    // This signer is not validated against the token account owner
    authority: Signer<'info>,
}
