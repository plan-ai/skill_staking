use crate::{
    event::BountyApplied,
    state::{Bounty, Freelancer},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApplyBounty<'info> {
    pub freelancer: Signer<'info>,
    #[account(
        seeds = [
            b"freelance",
            freelancer.key().as_ref(),
        ],
        bump=freelance_account.bump
    )]
    pub freelance_account: Account<'info, Freelancer>,
    #[account(
        mut,
        seeds = [
            b"bounty",
            bounty_account.bounty_creator.as_ref(),
            bounty_account.bounty_metadata.as_bytes()
        ],
        bump=bounty_account.bump
    )]
    pub bounty_account: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ApplyBounty>) -> Result<()> {
    let freelancer = &ctx.accounts.freelancer;
    let bounty_account = &mut ctx.accounts.bounty_account;

    bounty_account.bounty_appliers.push(freelancer.key());

    emit!(BountyApplied {
        bounty: bounty_account.key(),
        freelancer: freelancer.key()
    });

    Ok(())
}
