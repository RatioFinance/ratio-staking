use crate::*;

#[derive(Accounts)]
pub struct ResetFundingTime<'info> {
    #[account(mut, has_one = authority @ CommonError::Unauthorized)]
    pub reflection: Account<'info, ReflectionAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

impl<'info> ResetFundingTime<'info> {
    pub fn handler(&mut self) -> Result<()> {
        self.reflection.last_funding_time = Clock::get()?.unix_timestamp;
        Ok(())
    }
}
