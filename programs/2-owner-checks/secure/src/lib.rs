use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod owner_checks_secure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        // Unpack the token account data from the AccountInfo
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        
        // Ensure the token account is actually an SPL Token account
        if ctx.accounts.token.owner != &spl_token::ID {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // Check if the authority matches the token owner
        if ctx.accounts.authority.key != &token.owner {
            return Err(ProgramError::InvalidAccountData);
        }
        
        // Log the token amount
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    // Token account information
    token: AccountInfo<'info>,
    // Authority that must sign the transaction
    authority: Signer<'info>,
}
