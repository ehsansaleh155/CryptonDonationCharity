use anchor_lang::prelude::*;

#[account]
pub struct BaseAccount {
    pub platform_owner: Pubkey,
    pub period_n: i64,
    pub commission: u64,
    pub encrg_chrt: u32,
    pub lim_chrt_comm_exempt: u64,
    pub lim_chrt_camp_close: u32,
    pub account_size: u32, //account size which is set by the platform initializer.
                           //pub top_100_addresses: [Pubkey; 30], // adresses of top 30 donators of all time in platform (instead of 100)
                           //pub top_100_values: [u64; 30], // values of top 30 donators of all time in platform (instead of 100)
}

/* LEN is actually the account size which is now set by the platform initializer, but if we wanted to hardcode this, the codes below would be used
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
//const U8_LENGTH: usize = 1;
const F32_LENGTH: usize = 4;
//const U32_LENGTH: usize = 4;
//const U64_LENGTH: usize = 8;
*/

/*
impl BaseAccount {
    /*
    pub const LEN: usize = DISCRIMINATOR_LENGTH    // discriminator
            + PUBLIC_KEY_LENGTH                    // authority
            //+ TIMESTAMP_LENGTH                     // starting time
            + TIMESTAMP_LENGTH                     // period
            + F32_LENGTH                           // commission
            + F32_LENGTH                           // encouragement CHRT amount
            + F32_LENGTH                           // limit of the CHRT tokens for commission exemption
            + F32_LENGTH; // limit of the CHRT tokens for closure
        */
}
*/

//===================================================
#[account]
pub struct CampAccount {
    pub campaign_owner: Pubkey,
    pub starting_time: i64,
    pub commission_exempt: bool, //commission needs or not
                                 //pub top_10_addresses: [Pubkey; 10], // adresses of top 10 donators of all time in campaign
                                 //pub top_10_values: [u64; 10],       // values of top 10 donators of all time in campaign
}

//===================================================
#[account]
pub struct DonationData {
    pub commission_bank: Pubkey,
    pub donation_bank: Pubkey,
    pub referrer: Pubkey,
    pub donator: Pubkey,
    pub amount: u64,
}

//====================================================
