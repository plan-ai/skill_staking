use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod event;
pub mod helper;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CnMMyfQSGk7h6YNf2uLmBuLpfBKuMTYPct6PmFMM3P24");

#[program]
pub mod skill_staking {
    use super::*;

    pub fn create_name_router(
        ctx: Context<CreateNameRouter>,
        signing_domain: String,
        signature_version: u8,
    ) -> Result<()> {
        create_name_router::handler(ctx, signing_domain, signature_version)
    }

    pub fn add_verified_user(
        ctx: Context<AddVerifiedUser>,
        user_name: String,
        user_pubkey: Pubkey,
        msg: Vec<u8>,
        sig: [u8; 64],
    ) -> Result<()> {
        add_verified_user::handler(ctx, user_name, user_pubkey, msg, sig)
    }

    pub fn add_freelancer(ctx: Context<AddFreelancer>, freelancer_metadata: String) -> Result<()> {
        add_freelancer::handler(ctx, freelancer_metadata)
    }

    pub fn stake_skillset(
        ctx: Context<StakeSkillset>,
        skill: String,
        stake_amount: u64,
    ) -> Result<()> {
        stake_skillset::handler(ctx, skill, stake_amount)
    }

    pub fn create_bounty(
        ctx: Context<CreateBounty>,
        bounty_index: String,
        bounty_reward: u64,
        bounty_metadata: String,
        bounty_skillset: Vec<String>,
        bounty_deadline: Option<u64>,
    ) -> Result<()> {
        create_bounty::handler(
            ctx,
            bounty_index,
            bounty_reward,
            bounty_metadata,
            bounty_skillset,
            bounty_deadline,
        )
    }

    pub fn apply_bounty(ctx: Context<ApplyBounty>) -> Result<()> {
        apply_bounty::handler(ctx)
    }

    pub fn assign_freelancer(ctx: Context<AssignFreelancer>) -> Result<()> {
        assign_freelancer::handler(ctx)
    }

    pub fn accept_bounty(ctx: Context<AcceptBounty>) -> Result<()> {
        accept_bounty::handler(ctx)
    }

    pub fn claim_bounty(ctx: Context<ClaimBounty>) -> Result<()> {
        claim_bounty::handler(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        claim_reward::handler(ctx)
    }
}
