pub mod errors;
pub mod events;
pub mod state;
pub mod structures;
use crate::errors::DonationError;
use crate::events::*;
//use crate::state::*;
use crate::structures::*;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, rent, system_instruction};

declare_id!("FAJt5bV5epF8fYd9xzv7SqkapN4Bv5dsmvEfKxYYCvro");

#[program]
pub mod crypton_donation_charity {
    //use std::borrow::Borrow;

    use super::*;

    /*=====================================================================================*/
    //Initialize the platform
    pub fn initialize(
        ctx: Context<Initialize>,
        platform_owner: Pubkey,
        period_n: i64,
        commission: u64,
        encrg_chrt: u64,
        lim_chrt_comm_exempt: u64,
        lim_chrt_camp_close: u64,
        account_size: u32,
    ) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.platform_owner = platform_owner;
        base_account.period_n = period_n;
        base_account.commission = commission;
        base_account.encrg_chrt = encrg_chrt;
        base_account.lim_chrt_comm_exempt = lim_chrt_comm_exempt;
        base_account.lim_chrt_camp_close = lim_chrt_camp_close;
        base_account.account_size = account_size;
        base_account.total_donations = 0;
        base_account.plat_portion = 0;
        base_account.finished_camp_numbers = 0;

        let current_time = Clock::get()?.unix_timestamp;
        base_account.end_of_period = current_time + period_n;

        msg!("Platform Initialized!!!");
        Ok(())
    }
    /*=====================================================================================*/
    pub fn new_campaign(ctx: Context<NewCampaign>, campaign_owner: Pubkey) -> Result<()> {
        let camp_account = &mut ctx.accounts.camp_account;
        camp_account.campaign_owner = campaign_owner;
        camp_account.commission_exempt = false;
        camp_account.is_active = true;
        camp_account.don_number = 0;
        camp_account.com_number = 0;
        camp_account.chrt_token_recieved = 0;
        camp_account.camp_portion = 0;

        msg!("New Campaign Initialized!!!");
        Ok(())
    }
    /*=====================================================================================*/
    //receiver account is the referrer when calling the function
    pub fn do_donation(
        ctx: Context<DoDonation>,
        referrer: Pubkey,
        bump: u8,
        amount: u64,
    ) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account; //#5
        let camp_account = &mut ctx.accounts.camp_account; //#6 //need to match it with "campaign_owner"

        require!(camp_account.is_active, DonationError::CampaignFinished); //checks for campaign to be active

        let commission_bank = &mut ctx.accounts.commission_bank; //#1
        let donation_bank = &mut ctx.accounts.donation_bank; //#2
        let top_sto = &mut ctx.accounts.top_sto; //#7 // an account to keep track of top 100 in whole platform
        let donation_data = &mut ctx.accounts.donation_data; //#3
        let donator = &mut ctx.accounts.donator; //#4
        let receiver = &mut ctx.accounts.receiver; //###10 //account for referrer

        let commission = base_account.commission.clone();

        if !camp_account.commission_exempt {
            require!(amount > 0, DonationError::InvalidAmount);
            // Does the donator have enough lamports to transfer?
            let threshold = amount + commission;
            let don_lamp: u64 = donator.to_account_info().try_lamports()?;
            if don_lamp < threshold {
                return Err(DonationError::InsufficientFundsForTransaction.into());
            }
            structures::transfer_service_fee_lamports(
                &donator.to_account_info(),
                &commission_bank.to_account_info(),
                commission,
            )?;
            camp_account.com_number += 1;
            camp_account.don_number += 1;

            //check for commission exemption
            if camp_account.com_number as u64 * commission >= base_account.lim_chrt_comm_exempt {
                camp_account.commission_exempt = true;
            }
        } else {
            require!(amount > 0, DonationError::InvalidAmount);
            // Does the donator have enough lamports to transfer?
            let don_lamp: u64 = donator.to_account_info().try_lamports()?;
            if don_lamp < amount {
                return Err(DonationError::InsufficientFundsForTransaction.into());
            }
            camp_account.don_number += 1;
        }
        //=============================================
        invoke(
            &system_instruction::transfer(&donator.key(), &donation_bank.key(), amount),
            &[donator.to_account_info(), donation_bank.to_account_info()],
        )
        .map_err(Into::<error::Error>::into)?;
        //============================================

        donation_data
            .donation_camp
            .push(camp_account.to_account_info().key());
        donation_data.donator.push(donator.to_account_info().key());
        donation_data.referrer.push(referrer);
        donation_data.amount = donation_data.amount.saturating_add(amount);
        //============================================

        //top 10 camp keep track
        if camp_account.topten_camp_vlus.len() < 10 {
            camp_account.topten_camp_vlus.push(amount);
            camp_account
                .topten_camp_adrs
                .push(donator.to_account_info().key());
        } else {
            let min_val_camp_top_10 = camp_account.topten_camp_vlus.iter().min().unwrap().clone();
            if amount > min_val_camp_top_10 {
                let index = camp_account
                    .topten_camp_vlus
                    .iter()
                    .position(|x| x == &min_val_camp_top_10)
                    .unwrap();
                camp_account.topten_camp_vlus.remove(index);
                camp_account.topten_camp_vlus.push(amount);
                camp_account.topten_camp_adrs.remove(index);
                camp_account
                    .topten_camp_adrs
                    .push(donator.to_account_info().key());
            }
        }

        //top 10 plat keep track
        if base_account.topten_plat_vlus.len() < 10 {
            base_account.topten_plat_vlus.push(amount);
            base_account
                .topten_plat_adrs
                .push(donator.to_account_info().key());
        } else {
            let min_val_plat_top_10 = base_account.topten_plat_vlus.iter().min().unwrap().clone();
            if amount > min_val_plat_top_10 {
                let index = base_account
                    .topten_plat_vlus
                    .iter()
                    .position(|x| x == &min_val_plat_top_10)
                    .unwrap();
                base_account.topten_plat_vlus.remove(index);
                base_account.topten_plat_vlus.push(amount);
                base_account.topten_plat_adrs.remove(index);
                base_account
                    .topten_plat_adrs
                    .push(donator.to_account_info().key());
            }
        }

        //top 100 plat keep track
        if top_sto.top_100_values.len() < 100 {
            top_sto.top_100_values.push(amount);
            top_sto
                .top_100_addresses
                .push(donator.to_account_info().key());
        } else {
            let min_val_plat_top_100 = top_sto.top_100_values.iter().min().unwrap();
            if amount > *min_val_plat_top_100 {
                let index = top_sto
                    .top_100_values
                    .iter()
                    .position(|x| x == min_val_plat_top_100)
                    .unwrap();
                top_sto.top_100_values.remove(index);
                top_sto.top_100_values.push(amount);
                top_sto.top_100_addresses.remove(index);
                top_sto
                    .top_100_addresses
                    .push(donator.to_account_info().key());
            }
        }

        //============================================
        // TO DO CHRT Token to refferer
        let tamount = amount * 101; //CHRT token amount to send as gift

        let receiver_info = receiver.to_account_info();
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), //##11
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(), //##8
                    to: receiver_info,
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &[&[&"faucet-mint".as_bytes(), &[bump]]],
            ),
            tamount,
        )?;

        //============================================
        //Check for the end of period N and give token if needed
        let current_time = Clock::get()?.unix_timestamp;

        if current_time >= base_account.end_of_period {
            //IPLEMENT an error check for vectors not to be empty
            //
            emit!(TopTenRewardsEvent {
                top_addresses: base_account.topten_plat_adrs.to_vec(),
                top_values: base_account.topten_plat_vlus.to_vec(),
            });
            let mut destination = **ctx.accounts.destination; //##9

            while base_account.topten_plat_adrs.len() > 0 {
                destination.owner = base_account.topten_plat_adrs[0];

                anchor_spl::token::mint_to(
                    CpiContext::new_with_signer(
                        ctx.accounts.token_program.to_account_info(), //##11
                        anchor_spl::token::MintTo {
                            mint: ctx.accounts.mint.to_account_info(),
                            to: ctx.accounts.destination.to_account_info(), //&destination.to_account_info(), ////////////////////////////////////
                            authority: ctx.accounts.mint.to_account_info(),
                        },
                        &[&[&"faucet-mint".as_bytes(), &[bump]]],
                    ),
                    base_account.encrg_chrt,
                )?;

                base_account.topten_plat_adrs.remove(0);
                base_account.topten_plat_vlus.remove(0);
            }
            base_account.end_of_period = current_time + base_account.period_n;
        }

        //============================================
        emit!(DonationEvent {
            donation_bank: donation_bank.key(),
            donator: donator.key(),
            referrer: referrer,
            amount,
        });

        //============================================
        base_account.plat_portion += amount;
        camp_account.camp_portion += amount;

        //============================================
        msg!("One donation has been made!!!");
        msg!(
            "The total number of donations: {}",
            base_account.total_donations
        );
        msg!(
            "TOP-100 donations of the platform: Addresses: {:?} Values: {:?}",
            top_sto.top_100_addresses,
            top_sto.top_100_values
        );
        msg!(
            "TOP-10 donations of this campaign: Addresses: {:?} Values: {:?}",
            camp_account.topten_camp_adrs,
            camp_account.topten_camp_vlus
        );
        msg!(
            "The total amount of all donations of the platform {:?}",
            &ctx.accounts.donation_bank.to_account_info().try_lamports()
        );
        msg!(
            "The total amount of all donations of this campaign {}",
            camp_account.camp_portion
        );
        msg!(
            "The total amount of commission not paid to the owner {}",
            camp_account.don_number - camp_account.com_number
        );
        msg!(
            "The total amount of donations 'redistributed' to other campaings {}",
            base_account.finished_camp_numbers
        );

        Ok(())
    }
    /*=====================================================================================*/
    pub fn chrt_token_donation(
        ctx: Context<ChrtTokenDonation>,
        mint_bump: u8,
        amount: u64,
    ) -> Result<()> {
        let camp_account = &mut ctx.accounts.camp_account;
        require!(camp_account.is_active, DonationError::CampaignFinished); //checks for campaign to be active
        let base_account = &mut ctx.accounts.base_account;
        camp_account.chrt_token_recieved += amount;
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.mint.to_account_info(),
                },
                &[&[&"faucet-mint".as_bytes(), &[mint_bump]]],
            ),
            amount,
        )?;
        if camp_account.chrt_token_recieved >= base_account.lim_chrt_camp_close {
            camp_account.is_active = false;
            ////////// NEED TO IMPLEMENT DISTRIBUTION OF SOLs OF THIS CAMPAIGN BETWEEN OTHERS
        }

        msg!("CHRT Token donated!!!");
        Ok(())
    }
    /*=====================================================================================*/
    pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
        let rent_exempt_amount =
            rent::Rent::get()?.minimum_balance(ctx.accounts.base_account.account_size as usize);
        let bank = ctx.accounts.donation_bank.to_account_info();
        let initial_amount = bank.try_lamports()?;
        let portion =
            ctx.accounts.camp_account.camp_portion / ctx.accounts.base_account.plat_portion as u64;
        let mut amount = portion * initial_amount as u64;
        amount = amount.saturating_sub(rent_exempt_amount);
        ctx.accounts.base_account.plat_portion -= ctx.accounts.camp_account.camp_portion;
        ctx.accounts.base_account.finished_camp_numbers += 1;

        //require!(amount > 0, DonationError::NoFundsForWithdrawal);

        let destination = ctx.accounts.destination.to_account_info();

        **destination.try_borrow_mut_lamports()? += amount;
        **bank.try_borrow_mut_lamports()? = amount;
        //=======================================================================
        emit!(CloseCampaignEvent {
            donation_bank: ctx.accounts.donation_bank.key(),
            destination: ctx.accounts.destination.key(),
            amount,
        });
        Ok(())
    }
    /*=====================================================================================*/
    pub fn commission_withdrawal(ctx: Context<CommissionWithdrawal>) -> Result<()> {
        let rent_exempt_amount =
            rent::Rent::get()?.minimum_balance(ctx.accounts.base_account.account_size as usize);
        let bank = ctx.accounts.commission_bank.to_account_info();
        let amount = bank.try_lamports()?.saturating_sub(rent_exempt_amount);

        require!(amount > 0, DonationError::NoFundsForWithdrawal);

        let destination = ctx.accounts.destination.to_account_info();

        **destination.try_borrow_mut_lamports()? += amount;
        **bank.try_borrow_mut_lamports()? = amount;

        Ok(())
    }
    /*=====================================================================================*/
}
