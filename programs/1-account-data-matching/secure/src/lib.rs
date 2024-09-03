use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod account_data_matching_secure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        // Unpacking token account data
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        // Explicitly checking that the authority is the owner of the token account
        if ctx.accounts.authority.key != &token.owner {
            return Err(ProgramError::InvalidAccountData);
        }
        // Logging the account balance if ownership is verified
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    // Token account data needs to be carefully handled and ownership verified
    token: AccountInfo<'info>,
    // This signer is required to be the owner of the token account
    authority: Signer<'info>,
}
