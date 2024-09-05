use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

// Declare the program ID
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod type_cosplay_secure {
    use super::*;

    pub fn update_user(ctx: Context<UpdateUser>) -> ProgramResult {
        // Attempt to deserialize the user account data
        let user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        // Check if the owner of the account is the program itself
        if ctx.accounts.user.owner != ctx.program_id {
            return Err(ProgramError::IllegalOwner);
        }

        // Check if the authority matches the one stored in the user account
        if user.authority != ctx.accounts.authority.key() {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if the account has the correct discriminant to prevent type confusion
        if user.discriminant != AccountDiscriminant::User {
            return Err(ProgramError::InvalidAccountData);
        }

        // Log a message for debugging purposes
        msg!("GM {}", user.authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    user: AccountInfo<'info>, // User account (AccountInfo used instead of Account<User>)
    authority: Signer<'info>, // The signer who is expected to be the authority
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    discriminant: AccountDiscriminant, // Discriminant to distinguish between different account types
    authority: Pubkey,                 // Public key of the authority who can update the user
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Metadata {
    discriminant: AccountDiscriminant, // Discriminant to distinguish between different account types
    account: Pubkey,                   // Metadata account holding a public key
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AccountDiscriminant {
    User,      // Discriminant value for User accounts
    Metadata,  // Discriminant value for Metadata accounts
}
