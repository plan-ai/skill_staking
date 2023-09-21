use crate::{
    error::DefiOSError,
    event::SkillsetStaked,
    helper::find_index,
    state::{Freelancer, SkillStake},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(skill:String,stake_amount:u64)]
pub struct StakeSkillset<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"freelance",
            freelance_account.freelancer.as_ref(),
        ],
        bump = freelance_account.bump
    )]
    pub freelance_account: Account<'info, Freelancer>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = staker_token_account.owner.eq(&staker.key()),
        constraint = staker_token_account.amount >= stake_amount @ DefiOSError::InsufficientStakingFunds,
        constraint = staker_token_account.mint == usdc_mint.key()
    )]
    pub staker_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        space = 8+SkillStake::INIT_SPACE,
        payer = staker,
        seeds = [
            b"skillStake",
            skill.as_bytes(),
            freelance_account.freelancer.as_ref()
        ],
        bump
    )]
    pub skill_stake: Account<'info, SkillStake>,
    #[account(
        init_if_needed,
        payer = staker,
        associated_token::mint = usdc_mint,
        associated_token::authority = skill_stake,
    )]
    pub skill_stake_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<StakeSkillset>, skill: String, stake_amount: u64) -> Result<()> {
    let staker = &ctx.accounts.staker;
    let freelance_account = &mut ctx.accounts.freelance_account;
    let skill_stake = &mut ctx.accounts.skill_stake;
    let token_program = &ctx.accounts.token_program;
    let staker_token_account = &mut ctx.accounts.staker_token_account;
    let skill_stake_token_account = &mut ctx.accounts.skill_stake_token_account;

    skill_stake.freelancer = freelance_account.freelancer;
    skill_stake.skill = skill.clone();

    if let Some(index) = find_index(&skill_stake.stakers, &staker.key()) {
        skill_stake.stake_amounts[index] += stake_amount;
    } else {
        skill_stake.stake_amounts.push(stake_amount);
        skill_stake.stakers.push(staker.key());
        freelance_account.skills.push(skill.clone())
    }

    skill_stake.total_skill_stake += stake_amount;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: staker_token_account.to_account_info(),
                to: skill_stake_token_account.to_account_info(),
                authority: staker.to_account_info(),
            },
        ),
        stake_amount,
    )?;

    emit!(SkillsetStaked {
        staker: staker.key(),
        freelancer: freelance_account.freelancer.key(),
        skillset: skill,
        stake_amount: stake_amount,
    });

    Ok(())
}
