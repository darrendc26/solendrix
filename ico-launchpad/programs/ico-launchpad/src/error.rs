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
}