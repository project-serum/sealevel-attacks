use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod recommended {
    use super::*;

    pub fn check_sysvar_address(ctx: Context<CheckSysvarAddress>) -> Result<()> {
        // Logs the rent account's public key.
        // Here the rent account is properly typed as a Sysvar, ensuring it's a valid system account.
        msg!("Rent Key -> {}", ctx.accounts.rent.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckSysvarAddress<'info> {
    // Rent account is correctly specified as the Sysvar Rent, providing built-in validation.
    rent: Sysvar<'info, Rent>,
}
