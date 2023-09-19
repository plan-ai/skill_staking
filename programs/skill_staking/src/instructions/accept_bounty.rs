use crate::{
    constant::AUTHORIZED_PUBLIC_KEY,
    error::DefiOSError,
    event::FreelancerAssigned,
    state::{Bounty, Freelancer, Multisig},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AcceptBounty<'info> {
    pub first_signer: Signer<'info>,
    pub second_signer: Signer<'info>,
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
        seeds = [
            b"multisig",
            freelancer.key().as_ref(),
            bounty_account.bounty_creator.as_ref()
        ],
        bump=multisig.bump
    )]
    pub multisig: Account<'info, Multisig>,
    pub bounty_creator: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"bounty",
            bounty_creator.key().as_ref(),
            bounty_account.bounty_metadata.as_bytes()
        ],
        bump = bounty_account.bump,
        close = bounty_creator
    )]
    pub bounty_account: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AcceptBounty>) -> Result<()> {
    let first_signer = &ctx.accounts.first_signer;
    let second_signer = &mut ctx.accounts.second_signer;
    let multisig = &mut ctx.accounts.multisig;

    Ok(())
}
