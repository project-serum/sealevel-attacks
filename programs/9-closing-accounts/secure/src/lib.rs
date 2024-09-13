use anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;
use std::io::{Cursor, Write};
use std::ops::DerefMut;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod closing_accounts_secure {
    use super::*;

    pub fn close(ctx: Context<Close>) -> ProgramResult {
        let dest_starting_lamports = ctx.accounts.destination.lamports();

        let account = ctx.accounts.account.to_account_info();
        // Transfer lamports from the account being closed to the destination account
        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        // Clear the account data
        let mut data = account.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0;
        }

        // Write CLOSED_ACCOUNT_DISCRIMINATOR to mark the account as closed
        let dst: &mut [u8] = &mut data;
        let mut cursor = Cursor::new(dst);
        cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();

        Ok(())
    }

    pub fn force_defund(ctx: Context<ForceDefund>) -> ProgramResult {
        let account = &ctx.accounts.account;

        let data = account.try_borrow_data()?;
        assert!(data.len() > 8);

        // Check if the account is already closed by inspecting the discriminator
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&data[0..8]);
        if discriminator != CLOSED_ACCOUNT_DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }

        let dest_starting_lamports = ctx.accounts.destination.lamports();

        // Transfer remaining lamports from the account to the destination
        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    account: Account<'info, Data>, // The account to be closed
    destination: AccountInfo<'info>, // The account that receives the lamports
}

#[derive(Accounts)]
pub struct ForceDefund<'info> {
    account: AccountInfo<'info>, // The already-closed account
    destination: AccountInfo<'info>, // The account receiving any remaining lamports
}

#[account]
pub struct Data {
    data: u64,
}
