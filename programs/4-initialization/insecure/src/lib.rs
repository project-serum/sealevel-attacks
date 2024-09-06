use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::DerefMut;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod initialization_insecure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Deserialize the user account data
        let mut user = User::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        // Set the authority of the user
        user.authority = ctx.accounts.authority.key();

        // Re-serialize the user data back into the account
        let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
        user.serialize(storage.deref_mut()).unwrap();
        Ok(())
    }
}

/*
- reinitialize: The program can reinitialize the account, which may be a security risk.
- create and don't initialize: The account could be created without being initialized.
- passing previously initialized accounts from other programs (e.g., token program => need to check delegate and authority).
*/

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Unchecked user account, vulnerable to attacks like re-initialization
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    // Authority key of the user account
    authority: Pubkey,
}
