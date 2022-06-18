use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

/*============================================================================================================================initialize*/

#[derive(Accounts)]
#[instruction(platform_owner: Pubkey,)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = plat_payer, 
        space = 428 + 76 + 8, 
        seeds = [platform_owner.as_ref()], 
        bump)]
    pub base_account: Box<Account<'info, BaseAccount>>,
    #[account(mut)]
    pub plat_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/*============================================================================================================================new_campaign*/

#[derive(Accounts)]
#[instruction(
    campaign_owner: Pubkey, 
    starting_time: i64,)]
pub struct NewCampaign<'info> {
    #[account(
        init, 
        payer = camp_payer, 
        space = 457 + 8, 
        seeds = [campaign_owner.as_ref()], 
        bump)]
    pub camp_account: Account<'info, CampAccount>,
    #[account(mut)]
    pub camp_payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
/*============================================================================================================================do_donation*/

#[derive(Accounts)]
#[instruction(bump: u8, amount: u64)]
pub struct DoDonation<'info> {
    #[account(mut)]
    pub commission_bank: Account<'info, BaseAccount>, //for commission to platform//#1
    #[account(mut)]
    pub donation_bank: Account<'info, BaseAccount>, //for donation to campaign//#2
    #[account(
        init_if_needed, 
        payer = donator, 
        space = 64 + 1024, //not sure about this one :((
        seeds = [donator.key().as_ref(), campaign_owner.to_account_info().key.as_ref(), platform_owner.to_account_info().key.as_ref()], 
        bump)]
    pub donation_data: Account<'info, DonationData>,//#3
    #[account(mut)]
    pub donator: Signer<'info>,//#4
    #[account(mut)]
    pub platform_owner: AccountInfo<'info>, //for seed
    #[account(mut)]
    pub campaign_owner: AccountInfo<'info>, //for seed
    #[account(mut)]
    pub base_account: Box<Account<'info, BaseAccount>>,//#5
    #[account(mut)]
    pub camp_account: Box<Account<'info, CampAccount>>,//#6
    #[account(mut)]
    pub top_sto: Box<Account<'info, TopSto>>,//#7
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"faucet-mint".as_ref()],
        bump,
        mint::decimals = 3,
        mint::authority = mint
    )]
    pub mint: Account<'info, Mint>,//##8

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = adestination
    )]
    pub destination: Account<'info, TokenAccount>,//##9
    #[account(mut)]
    pub adestination: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = areceiver
    )]
    pub receiver: Account<'info, TokenAccount>,//###10
    #[account(mut)]
    pub areceiver: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,//##11
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

/*============================================================================================================================chrt_token_donation*/

#[derive(Accounts)]
#[instruction(mint_bump: u8, amount: u64)]
pub struct ChrtTokenDonation<'info> {
    #[account(mut)]
    pub base_account: Box<Account<'info, BaseAccount>>,
    #[account(mut)]
    pub camp_account: Box<Account<'info, CampAccount>>,
    #[account(
        mut,
        seeds = [b"fmint".as_ref()],
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

/*============================================================================================================================close_campaign*/ 

#[derive(Accounts)]
pub struct CloseCampaign<'info> {
    #[account(mut)]
    pub base_account: Box<Account<'info, BaseAccount>>,
    #[account(mut)]
    pub camp_account: Box<Account<'info, CampAccount>>,
    #[account(mut)]
    pub donation_bank: Account<'info, BaseAccount>, 
    pub platform_owner: Signer<'info>,
    #[account(mut)]
    pub destination: Account<'info, CampAccount>,
    #[account(mut)]
    pub bank: Account<'info, DonationData>,
}

/*============================================================================================================================commission_withdrawal*/ 

#[derive(Accounts)]
pub struct CommissionWithdrawal<'info> {
    #[account(mut)]
    pub base_account: Box<Account<'info, BaseAccount>>,
    #[account(mut)]
    pub commission_bank: Account<'info, BaseAccount>, 
    pub platform_owner: Signer<'info>,
    #[account(mut)]
    pub destination: Account<'info, BaseAccount>,
    #[account(mut)]
    pub bank: Account<'info, DonationData>,
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
