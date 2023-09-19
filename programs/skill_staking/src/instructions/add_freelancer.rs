use crate::{
    error::DefiOSError,
    event::FreelancerCreated,
    state::{Freelancer, VerifiedUser},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddFreelancer<'info> {
    #[account(
        mut,
        address = freelancer_verified_user.user_pubkey @ DefiOSError::UnauthorizedUser,
    )]
    pub freelancer: Signer<'info>,
    #[account(
        seeds = [
            freelancer_verified_user.user_name.as_bytes(),
            freelancer.key().as_ref(),
            freelancer_verified_user.name_router.key().as_ref()
        ],
        bump = freelancer_verified_user.bump
    )]
    pub freelancer_verified_user: Account<'info, VerifiedUser>,
    #[account(
        init,
        space = 8+Freelancer::INIT_SPACE,
        payer = freelancer,
        seeds = [
            b"freelance",
            freelancer.key().as_ref(),
        ],
        bump
    )]
    pub freelance_account: Account<'info, Freelancer>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddFreelancer>, freelancer_metadata: String) -> Result<()> {
    let freelancer = &ctx.accounts.freelancer;
    let freelance_account = &mut ctx.accounts.freelance_account;

    freelance_account.bump = *ctx.bumps.get("freelance_account").unwrap();
    freelance_account.freelancer = freelancer.key();
    freelance_account.user_metadata = freelancer_metadata.clone();

    emit!(FreelancerCreated {
        freelancer: freelancer.key(),
        freelancer_metadata: freelancer_metadata
    });

    Ok(())
}
