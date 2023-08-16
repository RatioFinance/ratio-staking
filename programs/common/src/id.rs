use anchor_lang::declare_id;

/***
 * IDs
 */

pub use system_program::ID as SYSTEM_PROGRAM;
mod system_program {
    use super::*;
    declare_id!("11111111111111111111111111111111");
}

pub use ratio_token::ID as RATIO_TOKEN;
mod ratio_token {
    use super::*;
    #[cfg(feature = "mainnet")]
    declare_id!("ratioMVg27rSZbSvBopUvsdrGUzeALUfFma61mpxc8J");
    #[cfg(not(feature = "mainnet"))]
    declare_id!("CaCXV7hMKsVKgiAd83Go8sCXDHzQ45ftfuiMGAg4TkGy");
}


pub use rewards_program::ID as REWARDS_PROGRAM;
mod rewards_program {
    use super::*;
    declare_id!("EkDHzRjupQbP2YayEN5j99D1482JbPxWLTz8MxzF7meN");
}

pub use staking_program::ID as STAKING_PROGRAM;
mod staking_program {
    use super::*;
    declare_id!("2YRmpDfPE619v42fgKwA7hSrcoUavNwcMfbcCToVj2qL");
}