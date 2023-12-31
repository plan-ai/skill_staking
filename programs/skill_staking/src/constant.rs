use anchor_lang::prelude::*;
use solana_program::pubkey;

#[constant]
pub const PROTOCOL_FEE: u64 = 12;

#[constant]
pub const BOUNTY_CREATOR_COMPENSATION: u64 = 2 * PROTOCOL_FEE;

#[constant]
pub const AUTHORIZED_PUBLIC_KEY: Pubkey = pubkey!("55kBY9yxqSC42boV8PywT2gqGzgLi5MPAtifNRgPNezF");
