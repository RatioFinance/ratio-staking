mod instructions;
mod macros;
mod state;

use anchor_lang::declare_id;
use anchor_lang::prelude::*;
use instructions::*;
use ratio_common::*;
pub use state::*; // expose state for cpi

declare_id!(id::REWARDS_PROGRAM);

#[program]
pub mod ratio_rewards {
    use super::*;

    /// Initialize the [ReflectionAccount](#reflection-account) and [VaultAccount](#vault-account).
    pub fn init(ctx: Context<Init>, funding_rate: f64) -> Result<()> {
        ctx.accounts.handler(*ctx.bumps.get("vault").unwrap(), funding_rate, *ctx.bumps.get("funding").unwrap())
    }

    /// Update funding rate. Can be only called by the authority stored in the reflection account.
    pub fn update_funding_rate(ctx: Context<UpdateFundingRate>, funding_rate: f64) -> Result<()> {
        ctx.accounts.handler(funding_rate)
    }

    /// Reset funding time. Can be only called by the authority stored in the reflection account.
    pub fn reset_funding_time(ctx: Context<ResetFundingTime>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Initialize a [RewardsAccount](#rewards-account).
    pub fn enter(ctx: Context<Enter>) -> Result<()> {
        ctx.accounts.handler(*ctx.bumps.get("reward").unwrap())
    }

    /// Send [NOS](/tokens/token) to the [VaultAccount](#vault-account).
    pub fn add_fee(ctx: Context<AddFee>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount)
    }

    /// Send estimated funding amount for the time elapsed from last funding time. [funding] -> [vault_account]. Can be called by anyone
    pub fn add_funding(ctx: Context<AddFunding>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Claim rewards from a [RewardsAccount](#rewards-account) and [VaultAccount](#vault-account).
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Re-calculate reflection points.
    pub fn sync(ctx: Context<Sync>) -> Result<()> {
        ctx.accounts.handler()
    }

    /// Close a [RewardsAccount](#rewards-account).
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.handler()
    }
}
