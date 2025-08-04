use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Ico {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub total_tokens: u64,
    pub price_lamports: u64,  
    pub is_active: bool,
    pub bump: u8,           
    // Optionally: pub merkle_root: [u8; 32], // For Light Protocol integration
}