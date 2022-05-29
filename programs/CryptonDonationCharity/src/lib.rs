pub mod errors;
pub mod events;
pub mod state;
pub mod structures;

use crate::structures::*;

use anchor_lang::prelude::*;

declare_id!("FAJt5bV5epF8fYd9xzv7SqkapN4Bv5dsmvEfKxYYCvro");

#[program]
pub mod crypton_donation_charity {
    use super::*;

    /*=====================================================================================*/
    //Initialize the platform
    pub fn initialize(
        ctx: Context<Initialize>,
        platform_owner: Pubkey,
        starting_time: i64,
        period_n: i64,
        commission: u64,
        encrg_chrt: u32,
        lim_chrt_comm_exempt: u64,
        lim_chrt_camp_close: u32,
        account_size: u32,
    ) -> Result<()> {
        structures::initialize(
            ctx,
            platform_owner,
            period_n,
            commission,
            encrg_chrt,
            lim_chrt_comm_exempt,
            lim_chrt_camp_close,
            account_size,
        )
    }
    /*=====================================================================================*/
    pub fn new_campaign(
        ctx: Context<NewCampaign>,
        campaign_owner: Pubkey,
        starting_time: i64,
    ) -> Result<()> {
        structures::new_campaign(ctx, campaign_owner, starting_time)
    }
    /*=====================================================================================*/
    pub fn do_donation(
        ctx: Context<DoDonation>,
        campaign_owner: Pubkey,
        referrer: Pubkey,
        mint_bump: u8,
        amount: u64,
    ) -> Result<()> {
        structures::do_donation(ctx, campaign_owner, referrer, mint_bump, amount)
    }
    /*=====================================================================================*/
    pub fn airdrop(
        ctx: Context<Airdrop>,
        receiver: Pubkey,
        payer: Pubkey,
        mint_bump: u8,
        amount: u64,
    ) -> Result<()> {
        structures::airdrop(ctx, receiver, payer, mint_bump, amount)
    }
    /*=====================================================================================*/
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        structures::withdraw(ctx)
    }
    /*======================================================================================*/
}
