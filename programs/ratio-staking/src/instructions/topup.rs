use crate::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Topup<'info> {
    #[account(mut)]
    pub user: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = vault @ RatioStakingError::InvalidVault,
        has_one = authority @ RatioStakingError::Unauthorized,
        constraint = stake.time_unstake == 0 @ RatioStakingError::AlreadyUnstaked,
    )]
    pub stake: Account<'info, StakeAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Topup<'info> {
    pub fn handler(&mut self, amount: u64) -> Result<()> {
        // test amount
        require!(amount > 0, RatioStakingError::AmountNotEnough);

        // get stake account and topup stake
        self.stake.topup(amount);

        // transfer tokens to the vault
        transfer_tokens_to_vault!(self, amount)
    }
}
