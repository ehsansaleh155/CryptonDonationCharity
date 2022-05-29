use anchor_lang::prelude::*;

#[error_code]
pub enum DonationError {
    #[msg("Amount should be more than zero!")]
    InvalidAmount,
    #[msg("The donation bank is empty")]
    NoFundsForWithdrawal,
    #[msg("The donation bank is empty")]
    InsufficientFundsForTransaction,
    //#[msg("Invalid Aggregator value returned")] //##
    //InvalidAggregatorValueReturned, //##
}
