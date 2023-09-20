use crate::{
    error::DefiOSError,
    event::BountyWon,
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
    #[account(
        mut,
        seeds = [
            b"bounty",
            bounty_account.bounty_creator.key().as_ref(),
            bounty_account.bounty_metadata.as_bytes()
        ],
        bump = bounty_account.bump,
    )]
    pub bounty_account: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AcceptBounty>) -> Result<()> {
    let first_signer = &ctx.accounts.first_signer;
    let second_signer = &mut ctx.accounts.second_signer;
    let multisig = &mut ctx.accounts.multisig;
    let bounty_account = &mut ctx.accounts.bounty_account;
    let freelancer = &ctx.accounts.freelancer;

    require!(
        multisig.owners.contains(&first_signer.key())
            && multisig.owners.contains(&second_signer.key()),
        DefiOSError::UnauthorizedActionAttempted
    );

    bounty_account.bounty_closed = true;

    emit!(BountyWon {
        bounty: bounty_account.key(),
        freelancer: freelancer.key(),
        bounty_reward: bounty_account.bounty_reward
    });

    Ok(())
}
