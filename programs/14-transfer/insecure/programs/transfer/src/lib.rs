use anchor_lang::prelude::*;

declare_id!("6usCzh8XM4LPL45zWkkPALz7jFCM4ojhhE5sjBTAW5JD");

#[program]
pub mod transfer {
    use super::*;

    pub fn add_balance(ctx: Context<AddBalance>, pubkey: Pubkey, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;

        user_account.balances.push((pubkey, amount));

        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, to: Pubkey, amount: u64) -> Result<()> {
        let mut to_account = ctx.accounts.to.clone();

        let sender_index = to_account
            .balances
            .iter()
            .position(|(pubkey, _)| *pubkey == ctx.accounts.from.key())
            .ok_or(ProgramError::UninitializedAccount)?;

        let sender_balance = &mut to_account.balances[sender_index].1;
        *sender_balance = sender_balance
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        let recipient_index = to_account
            .balances
            .iter()
            .position(|(pubkey, _)| *pubkey == to)
            .unwrap_or_else(|| {
                to_account.balances.push((to, 0));
                to_account.balances.len() - 1
            });

        let recipient_balance = &mut to_account.balances[recipient_index].1;
        *recipient_balance = recipient_balance
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        ctx.accounts.to = to_account;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init_if_needed, payer = authority, space = 8 + 4 + std::mem::size_of::<UserAccount>())]
    pub from: Account<'info, UserAccount>,
    #[account(init_if_needed, payer = authority, space = 8 + 4 + std::mem::size_of::<UserAccount>())]
    pub to: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub balances: Vec<(Pubkey, u64)>,
}

#[derive(Accounts)]
pub struct AddBalance<'info> {
    #[account(init_if_needed, payer = authority, space = 8 + 4 + std::mem::size_of::<UserAccount>())]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
