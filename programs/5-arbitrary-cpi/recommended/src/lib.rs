use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod arbitrary_cpi_recommended {
    use super::*;

    // Safe CPI invocation using verified token accounts
    pub fn cpi(ctx: Context<Cpi>, amount: u64) -> ProgramResult {
        // Uses anchor_spl's `transfer` helper, which performs more comprehensive checks on the accounts
        token::transfer(ctx.accounts.transfer_ctx(), amount)
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    // Strong typing for TokenAccount ensures proper token program interactions
    source: Account<'info, TokenAccount>,
    destination: Account<'info, TokenAccount>,
    // Verifies authority as a signer
    authority: Signer<'info>,
    // Program type enforces the correct program being used
    token_program: Program<'info, Token>,
}

impl<'info> Cpi<'info> {
    // Creates the context required for invoking the token transfer CPI
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Transfer {
            from: self.source.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        // Builds the CPI context for safe execution
        CpiContext::new(program, accounts)
    }
}
