use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Fundraiser {
    pub maker: Pubkey,
    pub amount_to_raise: u64,
    pub current_amount: u64,
    pub deadline: i64,
    pub bump: u8,
}
