use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token, Transfer, transfer};
use crate::ico::*;
use crate::error::ErrorCode;
use anchor_spl::associated_token::AssociatedToken;

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
    #[account(
        init,
        payer = owner,
        token::mint = token_mint,
        token::authority = ico,
        seeds = [b"vault", ico.key().as_ref()],
        bump,
    )]
    pub ico_vault: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = owner,
    )]
    pub owner_ata: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
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
    require!( start_time >= now, ErrorCode::StartTimeTooSoon );
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
    ico.total_invested = 0;
    ico.price_lamports = price_lamports;
    ico.total_raised = 0;
    ico.is_active = true;
    ico.bump = ctx.bumps.ico;

    // Transfer ownership of the token account to the ICO
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner_ata.to_account_info(),
        to: ctx.accounts.ico_vault.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_ctx, total_tokens)?;

    Ok(())
}