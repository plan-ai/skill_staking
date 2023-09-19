use crate::{event::BountyCreated, state::Bounty};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bounty_metadata: String)]
pub struct CreateBounty<'info> {
    #[account(mut)]
    pub bounty_creator: Signer<'info>,
    #[account(
        init,
        space = 8+Bounty::INIT_SPACE,
        payer = bounty_creator,
        seeds = [
            b"bounty",
            bounty_creator.key().as_ref(),
            bounty_metadata.as_bytes()
        ],
        bump
    )]
    pub bounty_account: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateBounty>,
    bounty_metadata: String,
    bounty_reward: u64,
    bounty_skillset: Vec<String>,
    bounty_deadline: Option<u64>,
) -> Result<()> {
    let bounty_creator = &ctx.accounts.bounty_creator;
    let bounty_account = &mut ctx.accounts.bounty_account;

    bounty_account.bump = *ctx.bumps.get("bump_account").unwrap();
    bounty_account.bounty_creator = bounty_creator.key();
    bounty_account.bounty_metadata = bounty_metadata.clone();
    bounty_account.bounty_reward = bounty_reward;
    bounty_account.bounty_skillset = bounty_skillset.clone();
    bounty_account.bounty_deadline = bounty_deadline;
    bounty_account.bounty_assigned = None;
    bounty_account.bounty_appliers = vec![];

    emit!(BountyCreated {
        bounty_creator: bounty_creator.key(),
        bounty_metadata: bounty_metadata,
        bounty_reward: bounty_reward,
        bounty_skillsets: bounty_skillset
    });

    Ok(())
}
