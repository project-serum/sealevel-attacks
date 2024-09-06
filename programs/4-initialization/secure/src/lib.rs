use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::DerefMut;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod reinitialization_secure_recommended {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Deserialize the user account data
        let mut user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        // Check if the account has been initialized by verifying the discriminator
        if !user.discriminator {
            return Err(ProgramError::InvalidAccountData);
        }

        // Set the authority and mark the account as initialized by setting discriminator to true
        user.authority = ctx.accounts.authority.key();
        user.discriminator = true;

        // Re-serialize the user data back into the account
        let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
        user.serialize(storage.deref_mut()).unwrap();

        // Log a message indicating successful initialization
        msg!("GM");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // User account with manual data checks for security
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    // Discriminator to ensure account has been initialized correctly
    discriminator: bool,
    // Authority key of the user account
    authority: Pubkey,
}
