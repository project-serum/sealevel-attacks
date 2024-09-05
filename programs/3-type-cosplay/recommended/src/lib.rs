use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

// Declare the program ID
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod type_cosplay_recommended {
    use super::*;

    pub fn update_user(ctx: Context<UpdateUser>) -> ProgramResult {
        // Log a message for debugging purposes
        msg!("GM {}", ctx.accounts.user.authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(has_one = authority)] // Ensures that the user account's authority matches the provided signer
    user: Account<'info, User>,     // User account with a strict type check
    authority: Signer<'info>,       // The signer who is expected to be the authority
}

#[account]
pub struct User {
    authority: Pubkey, // Public key of the authority who can update the user
}

#[account]
pub struct Metadata {
    account: Pubkey, // Metadata account holding a public key
}
