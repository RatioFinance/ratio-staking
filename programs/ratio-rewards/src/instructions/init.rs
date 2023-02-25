use crate::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(address = id::RATIO_TOKEN @ CommonError::InvalidMint)]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        space = ReflectionAccount::SIZE,
        seeds = [ constants::PREFIX_REFLECTION.as_ref() ],
        bump
    )]
    pub reflection: Account<'info, ReflectionAccount>,
    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = vault,
        seeds = [ mint.key().as_ref() ],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = funding,
        seeds = [ constants::PREFIX_FUNDING.as_ref() ],
        bump,
    )]
    pub funding: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Init<'info> {
    pub fn handler(&mut self, vault_bump: u8, funding_rate: f64, funding_bump: u8) -> Result<()> {
        self.reflection.init(self.vault.key(), vault_bump, funding_rate, funding_bump, self.authority.key())
    }
}
