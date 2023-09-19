use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod event;
pub mod helper;
pub mod instructions;
pub mod state;

declare_id!("CnMMyfQSGk7h6YNf2uLmBuLpfBKuMTYPct6PmFMM3P24");

#[program]
pub mod skill_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
