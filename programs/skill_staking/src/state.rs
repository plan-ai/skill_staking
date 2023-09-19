use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct NameRouter {
    pub bump: u8,
    pub signature_version: u8,
    pub total_verified_users: u64,
    pub router_creator: Pubkey,
    #[max_len(50)]
    pub signing_domain: String,
}

#[account]
#[derive(InitSpace)]
pub struct VerifiedUser {
    pub bump: u8,
    pub name_router: Pubkey,
    #[max_len(40)]
    pub user_name: String,
    pub user_pubkey: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Bounty {
    pub bump: u8,
    pub bounty_creator: Pubkey,
    #[max_len(100)]
    pub bounty_metadata: String,
    pub bounty_reward: u64,
    #[max_len(40, 40)]
    pub bounty_skillset: Vec<String>,
    pub bounty_deadline: Option<u64>,
    pub bounty_assigned: Option<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Freelancer {
    pub bump: u8,
    pub freelancer: Pubkey,
    #[max_len(100)]
    pub user_metadata: String,
}

#[account]
#[derive(InitSpace)]
pub struct SkillStake {
    pub bump: u8,
    pub freelancer: Pubkey,
    #[max_len(40)]
    pub skill: String,
    #[max_len(50)]
    pub stake_amounts: Vec<u64>,
    #[max_len(50)]
    pub stakers: Vec<Pubkey>,
    pub in_use: bool,
}
