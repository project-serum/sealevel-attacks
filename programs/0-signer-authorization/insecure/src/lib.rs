use anchor_lang::prelude::*;

// Insecure: `authority` is of type `AccountInfo` without any checks to ensure it's a signer.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod signer_authorization_insecure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: AccountInfo<'info>,  // No check for signer authority.
}
