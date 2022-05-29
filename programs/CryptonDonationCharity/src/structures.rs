use crate::errors::DonationError;
use crate::events::*;
use crate::state;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::rent;
use anchor_lang::solana_program::system_instruction;


/*============================================================================================================================initialize*/


pub fn initialize(
    ctx: Context<Initialize>,
    platform_owner: Pubkey,
    period_n: i64,
    commission: u64,
    encrg_chrt: u32,
    lim_chrt_comm_exempt: u64,
    lim_chrt_camp_close: u32,
    account_size: u32
) -> Result<()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.platform_owner = platform_owner;
    base_account.period_n = period_n;
    base_account.commission = commission;
    base_account.encrg_chrt = encrg_chrt;
    base_account.lim_chrt_comm_exempt = lim_chrt_comm_exempt;
    base_account.lim_chrt_camp_close = lim_chrt_camp_close;
    base_account.account_size = account_size;
    //base_account.top_100_addresses = [platform_owner; 30];
    //base_account.top_100_values = [0; 30];

    msg!("Platform Initialized!!!");
    Ok(())
}

//==============>>>>>>>>>>>>

/*
* 32 --> the size of the key,
* 8 ---> 8-byte-discriminator.
*/

#[derive(Accounts)]
#[instruction(platform_owner: Pubkey,)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = plat_payer, 
        space = 32 + 8, 
        seeds = [platform_owner.as_ref()], 
        bump)]
    pub base_account: Account<'info, state::BaseAccount>,
    #[account(mut)]
    pub plat_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/*============================================================================================================================new_campaign*/

pub fn new_campaign(
    ctx: Context<NewCampaign>,
    campaign_owner: Pubkey,
    starting_time: i64
) -> Result<()> {
    let camp_account = &mut ctx.accounts.camp_account;
    camp_account.campaign_owner = campaign_owner;
    camp_account.starting_time = starting_time;
    camp_account.commission_exempt = false;
    //camp_account.top_10_addresses = [campaign_owner; 10];
    //camp_account.top_10_values = [0; 10];

    msg!("New Campaign Initialized!!!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    campaign_owner: Pubkey, 
    starting_time: i64,)]
pub struct NewCampaign<'info> {
    #[account(
        init, 
        payer = camp_payer, 
        space = 32 + 8, 
        seeds = [campaign_owner.as_ref()], 
        bump)]
    pub camp_account: Account<'info, state::CampAccount>,
    #[account(mut)]
    pub camp_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
/*============================================================================================================================do_donation*/


pub fn do_donation(ctx: Context<DoDonation>, campaign_owner: Pubkey, referrer: Pubkey, bump: u8, amount: u64) -> Result<()> {
    if !ctx.accounts.camp_account.commission_exempt {
        require!(amount > 0, DonationError::InvalidAmount);
        // Does the donator have enough lamports to transfer?
        let commission = ctx.accounts.base_account.commission;
        let threshold = amount + commission;
        if **&ctx.accounts.donator.try_borrow_lamports()? < &mut threshold {
            return Err(DonationError::InsufficientFundsForTransaction.into());
        }

        transfer_service_fee_lamports(&ctx.accounts.donator.to_account_info(), &ctx.accounts.commission_bank.to_account_info(), commission)?;
    } else {
        require!(amount > 0, DonationError::InvalidAmount);
        // Does the donator have enough lamports to transfer?
        if **&ctx.accounts.donator.try_borrow_lamports()? < &mut amount {
            return Err(DonationError::InsufficientFundsForTransaction.into());
        }
    }

    

    //=============================================
    

    invoke(
        &system_instruction::transfer(
            &ctx.accounts.donator.key(),
            &ctx.accounts.donation_bank.key(),
            amount,
        ),
        &[
            ctx.accounts.donator.to_account_info(),
            ctx.accounts.donation_bank.to_account_info(),
        ],
    )
    .map_err(Into::<error::Error>::into)?;
    //============================================
    let donation_data = &mut ctx.accounts.donation_data;
    if donation_data.amount == 0 {
        donation_data.donator = ctx.accounts.donator.key();
        donation_data.donation_bank = ctx.accounts.donation_bank.key();
    }
    donation_data.amount = donation_data.amount.saturating_add(amount);
    //============================================
    /*
    //top 10 keep track
    let min_val_top_10 = ctx.accounts.camp_account.top_10_values.iter().min().unwrap();
    if amount > *min_val_top_10 {
        let index = ctx.accounts.camp_account.top_10_values.iter().position(|x| x == min_val_top_10).unwrap();
        ctx.accounts.camp_account.top_10_values.remove(index);
        ctx.accounts.camp_account.top_10_values.push(amount);
        ctx.accounts.camp_account.top_10_addresses.remove(index);
        ctx.accounts.camp_account.top_10_addresses.push(ctx.accounts.donator.key());
    }

    //top 100 keep track
    let min_val_top_100 = ctx.accounts.base_account.top_100_values.iter().min().unwrap();
    if amount > *min_val_top_100 {
        let index = ctx.accounts.base_account.top_100_values.iter().position(|x| x == min_val_top_100).unwrap();
        ctx.accounts.base_account.top_100_values.remove(index);
        ctx.accounts.base_account.top_100_values.push(amount);
        ctx.accounts.base_account.top_100_addresses.remove(index);
        ctx.accounts.base_account.top_100_addresses.push(ctx.accounts.donator.key());
    }
    */
    //============================================
    // TO DO CHRT Token to refferer
    let tamount = amount * 101;
    let receiver = &mut ctx.accounts.receiver;
    receiver.key = &referrer;
    let payer = &mut ctx.accounts.payer;
    payer.key = &ctx.accounts.base_account.platform_owner;
    let sender_info = ctx.accounts.mint.to_account_info();
    let receiver_info = ctx.accounts.destination.to_account_info();
    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: sender_info,
                to: receiver_info,
                authority: sender_info,
            },
            &[&[&"faucet-mint".as_bytes(), &[bump]]],
        ),
        tamount,
    )?;

    //============================================
    emit!(DonationEvent {
        donation_bank: ctx.accounts.donation_bank.key(),
        donator: ctx.accounts.donator.key(),
        referrer: referrer,
        amount,
        rewarded: false,
        campign_top_10: false,
        platform_top_100: false,
    });

    Ok(())
}

//==============>>>>>>>>>>>>
#[derive(Accounts)]
#[instruction(bump: u8, amount: u64)]
pub struct DoDonation<'info> {
    #[account(mut)]
    pub commission_bank: Account<'info, state::BaseAccount>, //for commission to platform
    #[account(mut)]
    pub donation_bank: Account<'info, state::CampAccount>, //for donation to campaign
    #[account(
        init_if_needed, 
        payer = donator, 
        space = 64 + 1024, 
        seeds = [donator.key().as_ref(), campaign_owner.to_account_info().key.as_ref(), platform_owner.to_account_info().key.as_ref()], 
        bump)]
    pub donation_data: Account<'info, state::DonationData>,
    #[account(mut)]
    pub donator: Signer<'info>,
    #[account(mut)]
    pub platform_owner: AccountInfo<'info>, //for seed
    #[account(mut)]
    pub campaign_owner: AccountInfo<'info>, //for seed
    #[account(mut)]
    pub base_account: Box<Account<'info, state::BaseAccount>>,
    #[account(mut)]
    pub camp_account: Account<'info, state::CampAccount>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"faucet-mint".as_ref()],
        bump,
        mint::decimals = 3,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = receiver
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

/*============================================================================================================================withdraw*/ 


pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let rent_exempt_amount =
        rent::Rent::get()?.minimum_balance(ctx.accounts.base_account.account_size as usize);
    let bank = ctx.accounts.donation_bank.to_account_info();
    let amount = bank.try_lamports()?.saturating_sub(rent_exempt_amount);

    require!(amount > 0, DonationError::NoFundsForWithdrawal);

    let destination = ctx.accounts.destination.to_account_info();

    **destination.try_borrow_mut_lamports()? += amount;
    **bank.try_borrow_mut_lamports()? = amount;
    //=======================================================================
    emit!(WithdrawEvent {
        donation_bank: ctx.accounts.donation_bank.key(),
        destination: ctx.accounts.destination.key(),
        amount,
    });
    Ok(())
}

//==============>>>>>>>>>>>>

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub donation_bank: Account<'info, state::CampAccount>, //for donation to campaign
    pub platform_owner: Signer<'info>,
    #[account(mut)]
    pub destination: Account<'info, state::CampAccount>,
    #[account(mut)]
    pub bank: Account<'info, state::DonationData>,
    #[account(mut)]
    pub base_account: Account<'info, state::BaseAccount>,
}

/*============================================================================================================================*/


/// Transfers lamports from one account (must be program owned)
/// to another account. The recipient can by any account
pub fn transfer_service_fee_lamports(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount_of_lamports: u64,
) -> Result<()> {
    // Debit from_account and credit to_account
    **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    Ok(())
}

/*
pub fn is_commission_exempt(plt_tkn_account_campaign: &AccountInfo) -> bool {
    // TO DO
    ctx.accounts.base_account.lim_chrt_comm_exempt =< plt_tkn_account_campaign
}
*/

/* 
/// Primary function handler associated with instruction sent
/// to your program
fn instruction_handler(accounts: &[AccountInfo]) -> Result<()> {
    // Get the 'from' and 'to' accounts
    let account_info_iter = &mut accounts.iter();
    let from_account = next_account_info(account_info_iter)?;
    let to_service_account = next_account_info(account_info_iter)?;

    // Extract a service 'fee' of 5 lamports for performing this instruction
    transfer_service_fee_lamports(from_account, to_service_account, 5u64)?;

    // Perform the primary instruction
    // ... etc.

    Ok(())
}
*/

/*============================================================================================================================token_airdrop*/
pub fn airdrop(ctx: Context<Airdrop>, receiver: Pubkey, payer: Pubkey, mint_bump: u8, amount: u64) -> Result<()> {
    let receiver = &mut ctx.accounts.receiver;
    //receiver.key = &receiver;
    let payer = &mut ctx.accounts.payer;
    //payer.key = &payer;
    let sender_info = ctx.accounts.mint.to_account_info();
    let receiver_info = ctx.accounts.destination.to_account_info();
    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: sender_info,
                to: receiver_info,
                authority: sender_info,
            },
            &[&[&"fmint".as_bytes(), &[mint_bump]]],
        ),
        amount,
    )?;

    msg!("Token airdroped!!!");
    Ok(())
}

#[derive(Accounts)]
#[instruction(mint_bump: u8, amount: u64)]
pub struct Airdrop<'info> {
    #[account(
        mut,
        seeds = [b"mint".as_ref()],
        bump = mint_bump,
        mint::decimals = 3,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = receiver
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account[mut]]
    pub payer: Signer<'info>,
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}