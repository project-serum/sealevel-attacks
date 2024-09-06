use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod reinitialization_4 {
    use super::*;

    pub fn init(_ctx: Context<Init>) -> ProgramResult {
        // Placeholder message for program initialization
        msg!("GM");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    // Initializes a new user account, allocating 8 bytes for the discriminator and 32 bytes for the Pubkey
    #[account(init, payer = authority, space = 8+32)]
    user: Account<'info, User>,
    
    // Authority is the payer of the transaction and must sign
    #[account(mut)]
    authority: Signer<'info>,
    
    // System program for allocating the account
    system_program: Program<'info, System>,
}

#[account]
pub struct User {
    // Authority key of the user account
    authority: Pubkey,
}
