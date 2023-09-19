use crate::{
    constant::AUTHORIZED_PUBLIC_KEY,
    error::DefiOSError,
    event::FreelancerAssigned,
    state::{Bounty, Freelancer},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AssignFreelancer<'info> {
    ///CHECK: This is not dangerous public key constraint is already set
    #[account(mut, signer,constraint=AUTHORIZED_PUBLIC_KEY.eq(&authority.key())@DefiOSError::UnauthorizedActionAttempted)]
    pub authority: AccountInfo<'info>,
    pub freelancer: SystemAccount<'info>,
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

pub fn handler(ctx: Context<AssignFreelancer>) -> Result<()> {
    let freelancer = &ctx.accounts.freelancer;
    let bounty_account = &mut ctx.accounts.bounty_account;

    bounty_account.bounty_assigned = Some(freelancer.key());

    emit!(FreelancerAssigned {
        bounty: bounty_account.key(),
        freelancer: freelancer.key()
    });

    Ok(())
}
