//use crate::state::Team;
use anchor_lang::prelude::*;

#[event]
pub struct DonationEvent {
    pub donation_bank: Pubkey,
    pub donator: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawEvent {
    pub donation_bank: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}

/*
#[event]
pub struct MatchResult {
    pub team: Team,
    pub nonce: u8,
    pub aggregator_key: Pubkey,
    pub arena_key: Pubkey,
}
*/
