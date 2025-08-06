use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Start time is too soon")]
    StartTimeTooSoon,
    #[msg("End time is too soon")]
    EndTimeTooSoon,
    #[msg("Total tokens is too low")]
    TotalTokensTooLow,
    #[msg("Price is too low")]
    PriceTooLow,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid price")]
    InvalidPrice,   
    #[msg("Invalid merkle tree index")]
    InvalidMerkleTreeIndex,
    #[msg("Account not enough keys")]
    AccountNotEnoughKeys,
    #[msg("Ico not active")]
    IcoNotActive,
    #[msg("Ico not started yet")]
    IcoNotStarted,
    #[msg("Ico ended already")]
    IcoEnded,
    #[msg("Not enough tokens to invest")]
    NotEnoughTokens,
    #[msg("Overflow error")]
    OverflowError,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}