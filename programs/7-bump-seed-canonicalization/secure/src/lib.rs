use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bump_seed_canonicalization_secure {
    use super::*;

    pub fn set_value_secure(
        ctx: Context<BumpSeed>,
        key: u64,
        new_value: u64,
        bump: u8,
    ) -> ProgramResult {
        // Safely derive the PDA and the correct bump seed using find_program_address
        let (address, expected_bump) =
            Pubkey::find_program_address(&[key.to_le_bytes().as_ref()], ctx.program_id);

        // Ensure the derived address matches the account key
        if address != ctx.accounts.data.key() {
            return Err(ProgramError::InvalidArgument);
        }
        // Ensure the expected bump matches the provided bump
        if expected_bump != bump {
            return Err(ProgramError::InvalidArgument);
        }

        // Update the value stored in the account
        ctx.accounts.data.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BumpSeed<'info> {
    data: Account<'info, Data>,  // The PDA account to be validated and used
}

#[account]
pub struct Data {
    value: u64,  // The value stored in the account
}
