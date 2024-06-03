use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct EightBoxState {
    pub user: Pubkey,
    pub start_time: u64,
    pub bet_amount: u64,
    pub claimed_status: u8,
}
