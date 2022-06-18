use anchor_lang::prelude::*;

#[error_code]
pub enum DonationError {
    #[msg("Amount should be more than zero!")]
    InvalidAmount,
    #[msg("The donation bank is empty!")]
    NoFundsForWithdrawal,
    #[msg("The donation bank is empty!")]
    InsufficientFundsForTransaction,
    #[msg("The speciffied campaign is finished!")]
    CampaignFinished,
    #[msg("The entered data for speciffied campaign is not correct!")]
    CampaignMismatch,
    //#[msg("Invalid Aggregator value returned")] //##
    //InvalidAggregatorValueReturned, //##
}
