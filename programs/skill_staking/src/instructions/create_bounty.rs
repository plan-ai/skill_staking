use crate::{error::DefiOSError, event::BountyCreated, state::Bounty};
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bounty_metadata: String,bounty_reward:u64)]
pub struct CreateBounty<'info> {
    #[account(mut)]
    pub bounty_creator: Signer<'info>,
    #[account(
        mut,
        constraint = bounty_creator_token_account.owner.eq(&bounty_creator.key()),
        constraint = bounty_creator_token_account.amount >= bounty_reward @ DefiOSError::InsufficientStakingFunds,
        constraint = bounty_creator_token_account.mint == usdc_mint.key()
    )]
    pub bounty_creator_token_account: Account<'info, TokenAccount>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
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
    #[account(
        init,
        payer = bounty_creator,
        associated_token::mint = usdc_mint,
        associated_token::authority = bounty_account,
    )]
    pub bounty_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateBounty>,
    bounty_metadata: String,
    bounty_reward: u64,
    bounty_skillset: Vec<String>,
    bounty_deadline: Option<u64>,
) -> Result<()> {
    let bounty_creator = &mut ctx.accounts.bounty_creator;
    let bounty_account = &mut ctx.accounts.bounty_account;
    let token_program = &ctx.accounts.token_program;
    let bounty_creator_token_account = &mut ctx.accounts.bounty_creator_token_account;
    let bounty_token_account = &mut ctx.accounts.bounty_token_account;

    bounty_account.bump = *ctx.bumps.get("bump_account").unwrap();
    bounty_account.bounty_creator = bounty_creator.key();
    bounty_account.bounty_metadata = bounty_metadata.clone();
    bounty_account.bounty_reward = bounty_reward;
    bounty_account.bounty_skillset = bounty_skillset.clone();
    bounty_account.bounty_deadline = bounty_deadline;
    bounty_account.bounty_assigned = None;
    bounty_account.bounty_appliers = vec![];
    bounty_account.claimed = vec![];

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: bounty_creator_token_account.to_account_info(),
                to: bounty_token_account.to_account_info(),
                authority: bounty_creator.to_account_info(),
            },
        ),
        bounty_reward,
    )?;

    emit!(BountyCreated {
        bounty_creator: bounty_creator.key(),
        bounty_metadata: bounty_metadata,
        bounty_reward: bounty_reward,
        bounty_skillsets: bounty_skillset
    });

    Ok(())
}
