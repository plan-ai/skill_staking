use crate::{
    constant::PROTOCOL_FEE,
    error::DefiOSError,
    event::RewardClaimed,
    helper::find_index,
    state::{Bounty, SkillStake},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
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
    // #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = staker_token_account.owner.eq(&staker.key()),
        constraint = staker_token_account.mint == usdc_mint.key()
    )]
    pub staker_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"skillStake",
            skill_stake.skill.as_bytes(),
            skill_stake.freelancer.as_ref()
        ],
        bump=skill_stake.bump
    )]
    pub skill_stake: Account<'info, SkillStake>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = skill_stake,
    )]
    pub skill_stake_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimReward>) -> Result<()> {
    let staker = &ctx.accounts.staker;
    let skill_stake = &mut ctx.accounts.skill_stake;
    let token_program = &ctx.accounts.token_program;
    let staker_token_account = &mut ctx.accounts.staker_token_account;
    let skill_stake_token_account = &mut ctx.accounts.skill_stake_token_account;
    let bounty_account = &mut ctx.accounts.bounty_account;
    let mut stake_amount: u64 = 0;

    if let Some(index) = find_index(&skill_stake.stakers, &staker.key()) {
        stake_amount = skill_stake.stake_amounts[index];
    } else {
        require!(1 == 0, DefiOSError::UnauthorizedActionAttempted);
    };

    require!(
        bounty_account.bounty_closed == true,
        DefiOSError::UnauthorizedActionAttempted
    );

    if let Some(_index) = find_index(&bounty_account.claimed, &staker.key()) {
        require!(1 == 0, DefiOSError::UnauthorizedActionAttempted);
    }

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"skillStake",
        skill_stake.skill.as_bytes(),
        skill_stake.freelancer.as_ref(),
        &[skill_stake.bump],
    ]];

    let reward_claim = (stake_amount / skill_stake.total_skill_stake)
        * (PROTOCOL_FEE / 100)
        * bounty_account.bounty_reward;
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: skill_stake_token_account.to_account_info(),
                to: staker_token_account.to_account_info(),
                authority: skill_stake.to_account_info(),
            },
            signer_seeds,
        ),
        reward_claim,
    )?;

    bounty_account.claimed.push(staker.key());

    emit!(RewardClaimed {
        bounty: bounty_account.key(),
        staker: staker.key(),
        claimed_amount: reward_claim
    });

    Ok(())
}
