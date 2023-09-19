use anchor_lang::prelude::*;

#[event]
pub struct NameRouterCreated {
    pub router_creator: Pubkey,
    pub name_router_account: Pubkey,
}

#[event]
pub struct VerifiedUserAdded {
    pub router_creator: Pubkey,
    pub name_router_account: Pubkey,
    pub verified_user_account: Pubkey,
    pub user_name: String,
    pub user_pubkey: Pubkey,
}

#[event]
pub struct FreelancerCreated {
    pub freelancer: Pubkey,
    pub freelancer_metadata: String,
}

#[event]
pub struct BountyCreated {
    pub bounty_creator: Pubkey,
    pub bounty_metadata: String,
    pub bounty_reward: u64,
    pub bounty_skillsets: Vec<String>,
}

#[event]
pub struct BountyDestroyed {
    pub bounty: Pubkey,
}

#[event]
pub struct SkillsetStaked {
    pub staker: Pubkey,
    pub freelancer: Pubkey,
    pub skillset: String,
    pub stake_amount: u64,
    pub in_use: bool,
}

#[event]
pub struct SkillsetUnStaked {
    pub skill: Pubkey,
    pub freelancer: Pubkey,
    pub unstake_amount: u64,
    pub unstaker: Pubkey,
}

#[event]
pub struct BountyFailed {
    pub freelancer: Pubkey,
    pub bounty: Pubkey,
    pub bounty_deadline: u64,
    pub skillsets: Vec<String>,
}

#[event]
pub struct BountyFailedClaimed {
    pub stake_amount: u64,
    pub redeemed_amount: u64,
    pub bounty: Pubkey,
}

#[event]
pub struct BountyWon {
    pub bounty: Pubkey,
    pub freelancer: Pubkey,
    pub bounty_reward: u64,
}

#[event]
pub struct BountyApplied {
    pub bounty: Pubkey,
    pub freelancer: Pubkey,
}

#[event]
pub struct FreelancerAssigned {
    pub bounty: Pubkey,
    pub freelancer: Pubkey,
}
