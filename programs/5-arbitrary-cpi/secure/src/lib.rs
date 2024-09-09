use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod arbitrary_cpi_secure {
    use super::*;

    // Secure CPI invocation with program ID validation
    pub fn cpi_secure(ctx: Context<Cpi>, amount: u64) -> ProgramResult {
        // Verifies the token program is indeed the spl_token program
        if &spl_token::ID != ctx.accounts.token_program.key {
            return Err(ProgramError::IncorrectProgramId); // Fail if not the correct program
        }
        // Safely invoke the transfer instruction
        solana_program::program::invoke(
            &spl_token::instruction::transfer(
                ctx.accounts.token_program.key,
                ctx.accounts.source.key,
                ctx.accounts.destination.key,
                ctx.accounts.authority.key,
                &[],
                amount,
            )?,
            &[
                ctx.accounts.source.clone(),
                ctx.accounts.destination.clone(),
                ctx.accounts.authority.clone(),
            ],
        )
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    // Same as the insecure version but with added program ID validation
    source: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}
