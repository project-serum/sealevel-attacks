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
    ) -> Result<()> {
        let (address, expected_bump) =
            Pubkey::find_program_address(&[key.to_le_bytes().as_ref()], ctx.program_id);

        if address != ctx.accounts.data.key() {
            return Err(ProgramError::InvalidArgument.into());
        }
        if expected_bump != bump {
            return Err(ProgramError::InvalidArgument.into());
        }

        ctx.accounts.data.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BumpSeed<'info> {
    data: Account<'info, Data>,
}

#[account]
pub struct Data {
    value: u64,
}
