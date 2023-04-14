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
    declare_id!("GAR1XhoN4SibcvnZNZt5fmaVk2smpqgnQ7HjxfoeX4MR");
}

pub use staking_program::ID as STAKING_PROGRAM;
mod staking_program {
    use super::*;
    declare_id!("A6kjpho36x7YvaodxXD7J6vZKBmrnzfpVUHFzN1qwUxr");
}