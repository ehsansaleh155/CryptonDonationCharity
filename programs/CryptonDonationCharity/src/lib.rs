pub mod errors;
pub mod events;
pub mod state;
pub mod structures;
use crate::errors::DonationError;
use crate::events::*;
use crate::state::*;
use crate::structures::*;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::rent;
use anchor_lang::solana_program::system_instruction;

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
        commission: f32,
        encrg_chrt: f32,
        lim_chrt_comm_exempt: f32,
        lim_chrt_camp_close: f32,
        account_size: u32,
    ) -> Result<()> {
        structures::initialize(
            ctx,
            platform_owner,
            starting_time,
            period_n,
            commission,
            encrg_chrt,
            lim_chrt_comm_exempt,
            lim_chrt_camp_close,
            account_size,
        )
    }
    /*=====================================================================================*/
    pub fn new_campaign(ctx: Context<NewCampaign>) -> Result<()> {
        structures::new_campaign(ctx)
    }

    /*=====================================================================================*/
    pub fn do_donation(ctx: Context<DoDonation>, mint_bump: u8, amount: u64) -> Result<()> {
        structures::do_donation(ctx, mint_bump, amount)
    }
    /*=====================================================================================*/
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        structures::withdraw(ctx)
    }
    /*======================================================================================*/
}
