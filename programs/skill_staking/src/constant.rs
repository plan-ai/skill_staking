use anchor_lang::prelude::*;

#[constant]
pub const PROTOCOL_FEE: u8 = 12;

#[constant]
pub const BOUNTY_CREATOR_COMPENSATION: u8 = 2 * PROTOCOL_FEE;
