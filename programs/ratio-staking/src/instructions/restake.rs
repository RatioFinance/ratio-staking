use crate::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct Restake<'info> {
    #[account(
        mut,
        constraint = vault.amount >= StakeAccount::STAKE_MINIMUM
            @ RatioStakingError::AmountNotEnough,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = vault @ CommonError::InvalidVault,
        has_one = authority @ CommonError::Unauthorized,
        constraint = stake.time_unstake != 0 @ RatioStakingError::AlreadyStaked,
    )]
    pub stake: Account<'info, StakeAccount>,
    pub authority: Signer<'info>,
}

impl<'info> Restake<'info> {
    pub fn handler(&mut self) -> Result<()> {
        self.stake.restake(self.vault.amount)
    }
}
