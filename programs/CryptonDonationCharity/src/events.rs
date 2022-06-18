use anchor_lang::prelude::*;

#[event]
pub struct DonationEvent {
    pub donation_bank: Pubkey,
    pub donator: Pubkey,
    pub referrer: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TopTenRewardsEvent {
    pub top_addresses: Vec<Pubkey>,
    pub top_values: Vec<u64>,
}

#[event]
pub struct CloseCampaignEvent {
    pub donation_bank: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}
