use crate::{
    constant::AUTHORIZED_PUBLIC_KEY,
    error::DefiOSError,
    event::FreelancerAssigned,
    state::{Bounty, Freelancer, Multisig},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AssignFreelancer<'info> {
    ///CHECK: This is not dangerous public key constraint is already set
    #[account(mut, signer)]
    // constraint=AUTHORIZED_PUBLIC_KEY.eq(&authority.key())@DefiOSError::UnauthorizedActionAttempted)]
    pub authority: AccountInfo<'info>,
    #[account(
        seeds = [
            b"freelance",
            freelance_account.freelancer.as_ref(),
        ],
        bump=freelance_account.bump
    )]
    pub freelance_account: Account<'info, Freelancer>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8+Multisig::INIT_SPACE,
        seeds = [
            b"multi_sig",
            freelance_account.freelancer.as_ref(),
            bounty_account.bounty_creator.as_ref()
        ],
        bump,
    )]
    pub multi_sig: Account<'info, Multisig>,
    #[account(
        mut,
        seeds = [
            b"bounty",
            bounty_account.bounty_creator.as_ref(),
            bounty_account.index.as_bytes()
        ],
        bump=bounty_account.bump
    )]
    pub bounty_account: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AssignFreelancer>) -> Result<()> {
    let freelancer = ctx.accounts.freelance_account.freelancer;
    let bounty_account = &mut ctx.accounts.bounty_account;
    let multi_sig = &mut ctx.accounts.multi_sig;

    bounty_account.bounty_assigned = Some(freelancer.key());

    multi_sig.bump = *ctx.bumps.get("multi_sig").unwrap();
    multi_sig.owners = vec![
        freelancer,
        AUTHORIZED_PUBLIC_KEY,
        bounty_account.bounty_creator,
    ];
    multi_sig.threshold = 2;

    emit!(FreelancerAssigned {
        bounty: bounty_account.key(),
        freelancer: freelancer
    });

    Ok(())
}
