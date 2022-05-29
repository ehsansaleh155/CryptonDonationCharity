use anchor_lang::prelude::*;

#[event]
pub struct DonationEvent {
    pub donation_bank: Pubkey,
    pub donator: Pubkey,
    pub referrer: Pubkey,
    pub amount: u64,
    pub rewarded: bool,
    pub campign_top_10: bool,
    pub platform_top_100: bool,
}

#[event]
pub struct WithdrawEvent {
    pub donation_bank: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}
