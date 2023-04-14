use crate::*;

#[derive(Accounts)]
pub struct UpdateFundingRate<'info> {
    #[account(mut, has_one = authority @ CommonError::Unauthorized)]
    pub reflection: Account<'info, ReflectionAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

impl<'info> UpdateFundingRate<'info> {
    pub fn handler(&mut self, funding_rate: f64) -> Result<()> {
        self.reflection.funding_rate = funding_rate;
        Ok(())
    }
}
