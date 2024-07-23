use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod owner_checks_secure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow()).unwrap();
        if ctx.accounts.token.owner.to_string() != spl_token::ID.to_string() {
            return Err(ProgramError::InvalidAccountData.into());
        }
        if ctx.accounts.authority.key.to_string() != token.owner.to_string() {
            return Err(ProgramError::InvalidAccountData.into());
        }
        msg!("Your account balance is: {}", token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}
