use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::system_instruction::transfer;
use anchor_spl::token::spl_token;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod arbitrary_cpi_secure {
    use super::*;

    pub fn cpi_secure(ctx: Context<Cpi>, amount: u64) -> Result<()> {
        if spl_token::ID.to_string() != ctx.accounts.token_program.key.to_string() {
            return Err(ProgramError::IncorrectProgramId.into());
        }
        Ok(solana_program::program::invoke(
            &transfer(
                ctx.accounts.source.key,
                ctx.accounts.destination.key,
                amount,
            ),
            &[
                ctx.accounts.source.clone(),
                ctx.accounts.destination.clone(),
                ctx.accounts.authority.clone(),
            ],
        )?)
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    source: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
}
