use crate::{
    event::SkillsetStaked,
    helper::find_index,
    state::{Freelancer, SkillStake},
};
use anchor_lang::prelude::*;

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
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<StakeSkillset>, skill: String, stake_amount: u64) -> Result<()> {
    let staker = &ctx.accounts.staker;
    let freelance_account = &mut ctx.accounts.freelance_account;
    let skill_stake = &mut ctx.accounts.skill_stake;

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

    emit!(SkillsetStaked {
        staker: staker.key(),
        freelancer: freelance_account.freelancer.key(),
        skillset: skill,
        stake_amount: stake_amount,
    });

    Ok(())
}
