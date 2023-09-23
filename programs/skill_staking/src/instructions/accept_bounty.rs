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
    #[account(
        seeds = [
            b"freelance",
            freelance_account.freelancer.as_ref(),
        ],
        bump=freelance_account.bump
    )]
    pub freelance_account: Account<'info, Freelancer>,
    #[account(
        seeds = [
            b"multi_sig",
            freelance_account.freelancer.as_ref(),
            bounty_account.bounty_creator.as_ref()
        ],
        bump=multi_sig.bump
    )]
    pub multi_sig: Account<'info, Multisig>,
    #[account(
        mut,
        seeds = [
            b"bounty",
            bounty_account.bounty_creator.key().as_ref(),
            bounty_account.index.as_bytes()
        ],
        bump = bounty_account.bump,
    )]
    pub bounty_account: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AcceptBounty>) -> Result<()> {
    let first_signer = &ctx.accounts.first_signer;
    let second_signer = &mut ctx.accounts.second_signer;
    let multi_sig = &mut ctx.accounts.multi_sig;
    let freelance_account = &ctx.accounts.freelance_account;
    let bounty_account = &mut ctx.accounts.bounty_account;

    require!(
        multi_sig.owners.contains(&first_signer.key())
            && multi_sig.owners.contains(&second_signer.key())
            && bounty_account.bounty_assigned.unwrap() == freelance_account.freelancer,
        DefiOSError::UnauthorizedActionAttempted
    );

    bounty_account.bounty_closed = true;

    emit!(BountyWon {
        bounty: bounty_account.key(),
        freelancer: freelance_account.freelancer,
        bounty_reward: bounty_account.bounty_reward
    });

    Ok(())
}
