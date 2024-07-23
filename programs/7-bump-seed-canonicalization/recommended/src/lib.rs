use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bump_seed_canonicalization_recommended {
    use super::*;

    pub fn set_value(ctx: Context<BumpSeed>, key: u64, new_value: u64) -> Result<()> {
        ctx.accounts.data.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct BumpSeed<'info> {
    // Note a subtle pattern that is not displayed here.
    //
    // Usually, the usage of PDAs is broken into two parts:
    //
    // 1) allocation via `#[account(init, seeds = [...], bump)]`
    // 2) using the account via `#[account(init, seeds = [...], bump = data.bump)]
    //
    // When using a PDA, it's usually recommend to store the bump seed in the
    // account data, so that you can use it as demonstrated in 2), which will
    // provide a more efficient check.
    #[account(seeds = [key.to_le_bytes().as_ref()], bump)]
    data: Account<'info, Data>,
}

#[account]
pub struct Data {
    value: u64,
}
