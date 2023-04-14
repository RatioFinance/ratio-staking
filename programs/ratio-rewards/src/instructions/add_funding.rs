use crate::*;
use anchor_spl::token::{Token, TokenAccount};
use ratio_common::constants::{PREFIX_FUNDING};

#[derive(Accounts)]
pub struct AddFunding<'info> {
    #[account(mut, 
        seeds = [ constants::PREFIX_FUNDING.as_ref()],
        bump
    )]
    pub funding: Account<'info, TokenAccount>,
    #[account(mut, has_one = vault @ CommonError::InvalidVault)]
    pub reflection: Account<'info, ReflectionAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> AddFunding<'info> {
    pub fn handler(&mut self) -> Result<()> {
        let funding_seeds = [PREFIX_FUNDING.as_bytes(), &[self.reflection.funding_bump]];
        let signer_seeds = &[&funding_seeds[..]][..];

        // Calculate the total amount of funding to be processed since last funding time
        let current_time = Clock::get()?.unix_timestamp;
        let time_elapsed_in_seconds = current_time
            .checked_sub(self.reflection.last_funding_time)
            .unwrap();
        let total_amount =
            (self.reflection.funding_rate * time_elapsed_in_seconds as f64).floor();
        if total_amount as u64 == 0 {
            Ok(())
        } else {
            // Reset Last Funding Time
            self.reflection.last_funding_time = current_time;
    
            self.reflection.add_fee(u128::from(total_amount as u64));
            transfer_tokens_from_funding!(self, vault, signer_seeds, total_amount as u64)
        }

    }
}
