use anchor_lang::prelude::*;

use crate::ico::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitializeIco<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Ico::INIT_SPACE,
        seeds = ["ico".as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub ico: Account<'info, Ico>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_ico_handler(
    ctx: Context<InitializeIco>,
    token_mint: Pubkey,
    token_account: Pubkey,
    start_time: i64,
    end_time: i64,
    total_tokens: u64,
    price_lamports: u64,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!( start_time > now, ErrorCode::StartTimeTooSoon );
    require!( end_time > start_time, ErrorCode::EndTimeTooSoon );
    require!( end_time > now, ErrorCode::EndTimeTooSoon );
    require!( total_tokens > 0, ErrorCode::TotalTokensTooLow );
    require!( price_lamports > 0, ErrorCode::PriceTooLow );

    let ico = &mut ctx.accounts.ico;
    ico.owner = ctx.accounts.owner.key();
    ico.token_mint = token_mint;
    ico.token_account = token_account;
    ico.start_time = start_time;
    ico.end_time = end_time;
    ico.total_tokens = total_tokens;
    ico.price_lamports = price_lamports;
    ico.total_raised = 0;
    ico.is_active = true;
    ico.bump = ctx.bumps.ico;

    Ok(())
}