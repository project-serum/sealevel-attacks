use anchor_lang::prelude::*;
use std::io::Write;
use std::ops::DerefMut;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod closing_accounts_insecure_still_still {
    use super::*;

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let account = ctx.accounts.account.to_account_info();

        let dest_starting_lamports = ctx.accounts.destination.lamports();

        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        let mut data = account.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0;
        }

        let dst: &mut [u8] = &mut data;
        let mut cursor = std::io::Cursor::new(dst);
        cursor
            .write_all(&[0; 8])
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    account: Account<'info, Data>,
    destination: AccountInfo<'info>,
}

#[account]
pub struct Data {
    data: u64,
}
