use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bump_seed_canonicalization_insecure {
    use super::*;

    pub fn set_value(ctx: Context<BumpSeed>, key: u64, new_value: u64, bump: u8) -> ProgramResult {
        // Derive the PDA using the provided bump seed
        let address =
            Pubkey::create_program_address(&[key.to_le_bytes().as_ref(), &[bump]], ctx.program_id)?;
        
        // Ensure the derived address matches the account key
        if address != ctx.accounts.data.key() {
            return Err(ProgramError::InvalidArgument);
        }

        // Update the value stored in the account
        ctx.accounts.data.value = new_value;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BumpSeed<'info> {
    data: Account<'info, Data>,  // The account where the data will be stored
}

#[account]
pub struct Data {
    value: u64,  // The value stored in the account
}
