use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bump_seed_canonicalization_recommended {
    use super::*;

    pub fn set_value(ctx: Context<BumpSeed>, key: u64, new_value: u64) -> ProgramResult {
        // Simply update the value stored in the account, using PDA in a safe manner
        ctx.accounts.data.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct BumpSeed<'info> {
    // Use the key to generate the PDA and store the bump seed in the account for future use.
    #[account(seeds = [key.to_le_bytes().as_ref()], bump)]
    data: Account<'info, Data>,  // The PDA account to be validated and used
}

#[account]
pub struct Data {
    value: u64,  // The value stored in the account
}
