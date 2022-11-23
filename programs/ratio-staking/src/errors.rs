use anchor_lang::prelude::*;

/***
 * Errors
 */

#[error_code]
pub enum RatioStakingError {
    #[msg("This amount is not enough.")]
    AmountNotEnough,
    #[msg("This stake is already running.")]
    AlreadyInitialized,
    #[msg("This stake is already claimed.")]
    AlreadyClaimed,
    #[msg("This stake is already staked.")]
    AlreadyStaked,
    #[msg("This stake is already unstaked.")]
    AlreadyUnstaked,
    #[msg("This stake is not yet unstaked.")]
    NotUnstaked,
    #[msg("This stake is still locked.")]
    Locked,
    #[msg("This stake duration is not long enough.")]
    DurationTooShort,
    #[msg("This stake duration is too long.")]
    DurationTooLong,
    #[msg("This stake account does not exist.")]
    DoesNotExist,
    #[msg("This stake is not allowed to decrease.")]
    Decreased,
    #[msg("This stake still has a reward account.")]
    HasReward,
    #[msg("This stake does not belong to the authority.")]
    InvalidStakeAccount,

    // generic errors
    #[msg("This account is not authorized to perform this action.")]
    Unauthorized,
    #[msg("This account is owned by an invalid program.")]
    InvalidOwner,
    #[msg("This account has lamports.")]
    LamportsNonNull,
    #[msg("This account is missing a signature.")]
    MissingSignature,
    #[msg("This account is not valid.")]
    InvalidAccount,
    #[msg("This token account is not valid.")]
    InvalidTokenAccount,
    #[msg("This mint is invalid.")]
    InvalidMint,
    #[msg("This account has an invalid vault.")]
    InvalidVault,
    #[msg("This payer account is not valid.")]
    InvalidPayer,
    #[msg("This vault is empty.")]
    VaultEmpty,
    #[msg("This vault is not empty.")]
    VaultNotEmpty,
}
