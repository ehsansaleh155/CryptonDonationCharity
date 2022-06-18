//use crate::borsh::maybestd::rc::Rc;
use anchor_lang::prelude::*;

#[account]
pub struct BaseAccount {
    pub platform_owner: Pubkey,
    pub period_n: i64,
    pub end_of_period: i64,
    pub commission: u64,
    pub encrg_chrt: u64,
    pub lim_chrt_comm_exempt: u64,
    pub lim_chrt_camp_close: u64,
    pub account_size: u32, //account size which is set by the platform initializer.
    pub topten_plat_adrs: Vec<Pubkey>,
    pub topten_plat_vlus: Vec<u64>,
    pub total_donations: u64,
    pub plat_portion: u64,
    pub finished_camp_numbers: u32,
}

//===================================================
#[account]
pub struct CampAccount {
    pub campaign_owner: Pubkey,
    pub commission_exempt: bool, //commission needs or not
    pub is_active: bool,
    pub topten_camp_adrs: Vec<Pubkey>, // adresses of top 10 donators in campaign
    pub topten_camp_vlus: Vec<u64>,    // values of top 10 donators in campaign
    pub don_number: i32,
    pub com_number: i32,
    pub chrt_token_recieved: u64,
    pub camp_portion: u64,
}

//===================================================
#[account]
pub struct DonationData {
    pub donation_camp: Vec<Pubkey>,
    pub referrer: Vec<Pubkey>,
    pub donator: Vec<Pubkey>,
    pub amount: u64,
}

//====================================================

#[account]
pub struct TopSto {
    pub top_100_addresses: Vec<Pubkey>,
    pub top_100_values: Vec<u64>,
}
