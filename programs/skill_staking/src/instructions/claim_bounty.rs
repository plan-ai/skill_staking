use crate::{constant::PROTOCOL_FEE, error::DefiOSError, event::BountyClaimed, state::Bounty};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{
        create as create_associated_token_account, get_associated_token_address, AssociatedToken,
        Create,
    },
    mint::USDC,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    pub claimer: Signer<'info>,
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
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = bounty_account
    )]
    pub bounty_token_account: Account<'info, TokenAccount>,
    /// CHECK: PDA check is done at the handler function
    #[account(mut)]
    pub claimer_token_account: UncheckedAccount<'info>,
    #[account(address=USDC)]
    pub usdc_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimBounty>) -> Result<()> {
    let claimer = &mut ctx.accounts.claimer;
    let claimer_token_account = &mut ctx.accounts.claimer_token_account;
    let bounty_account = &mut ctx.accounts.bounty_account;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let bounty_token_account = &mut ctx.accounts.bounty_token_account;
    let usdc_mint = &ctx.accounts.usdc_mint;

    require!(
        bounty_account.bounty_closed == true
            && bounty_account.bounty_assigned == Some(claimer.key()),
        DefiOSError::UnauthorizedActionAttempted
    );

    //Creating token account if empty
    if claimer_token_account.data_is_empty() {
        create_associated_token_account(CpiContext::new(
            associated_token_program.to_account_info(),
            Create {
                payer: claimer.to_account_info(),
                associated_token: claimer_token_account.to_account_info(),
                authority: claimer.to_account_info(),
                mint: usdc_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                token_program: token_program.to_account_info(),
            },
        ))?;
    }

    //checking if issue token account sent is same as expected

    let expected_claimer_token_account =
        get_associated_token_address(&claimer.key(), &usdc_mint.key());

    require!(
        expected_claimer_token_account.eq(&claimer_token_account.key()),
        DefiOSError::TokenAccountMismatch
    );

    let bounty_account_key = bounty_account.bounty_creator;
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"bounty",
        bounty_account_key.as_ref(),
        bounty_account.bounty_metadata.as_bytes(),
        &[bounty_account.bump],
    ]];

    let claimed_amount = bounty_account.bounty_reward * (100 - PROTOCOL_FEE) / 100;
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: bounty_token_account.to_account_info(),
                to: claimer_token_account.to_account_info(),
                authority: bounty_account.to_account_info(),
            },
            signer_seeds,
        ),
        claimed_amount,
    )?;

    emit!(BountyClaimed {
        bounty: bounty_account.key(),
        freelancer: claimer.key(),
        claimed_amount: claimed_amount
    });

    Ok(())
}
