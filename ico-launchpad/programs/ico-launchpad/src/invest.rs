use anchor_lang::prelude::*;
use light_sdk::{
    cpi::{CpiAccounts, CpiInputs, CpiSigner},
    instruction::{PackedAddressTreeInfo, ValidityProof},
    derive_light_cpi_signer,
    LightDiscriminator,
    LightHasher,
};
use anchor_lang::solana_program;
use crate::error::ErrorCode;
use crate::ico::*;

pub const LIGHT_CPI_SIGNER: CpiSigner =
    derive_light_cpi_signer!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
pub const LIGHT_PROTOCOL_ID: Pubkey = solana_program::pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Accounts)]
pub struct Invest<'info> {
    #[account(mut, has_one = owner)]
    pub ico: Account<'info, Ico>,
    
    #[account(mut)]
    pub owner: AccountInfo<'info>,
    
    #[account(mut)]
    pub investor: Signer<'info>,
    
    /// CHECK: Light Protocol program validation
    #[account(address = LIGHT_PROTOCOL_ID)]
    pub light_protocol_program: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    
    /// CHECK: Token program for potential token transfers
    pub token_program: AccountInfo<'info>,
    
    /// CHECK: Associated token program
    pub associated_token_program: AccountInfo<'info>,
}

pub fn invest_handler<'info>(
    ctx: Context<'_, '_, '_, 'info, Invest<'info>>,
    proof: ValidityProof,
    address_tree_info: PackedAddressTreeInfo,
    output_merkle_tree_index: u8,
    amount: u64,
) -> Result<()> {
    let ico = &mut ctx.accounts.ico;
    let now = Clock::get()?.unix_timestamp;
    
    // Validation checks
    require!(ico.is_active, ErrorCode::IcoNotActive);
    require!(now >= ico.start_time, ErrorCode::IcoNotStarted);
    require!(now <= ico.end_time, ErrorCode::IcoEnded);
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(ico.total_tokens >= amount, ErrorCode::NotEnoughTokens);
    
    // Validate output merkle tree index is within bounds
    require!(output_merkle_tree_index < 32, ErrorCode::InvalidMerkleTreeIndex);
    
    // Setup Light Protocol CPI accounts
    let light_cpi_accounts = CpiAccounts::new(
        &ctx.accounts.investor,
        ctx.remaining_accounts,
        LIGHT_CPI_SIGNER,
    );
    
    // Derive the shielded address for this investment
    let (address, address_seed) = light_sdk::address::v1::derive_address(
        &[
            b"allocation", 
            ctx.accounts.investor.key().as_ref(),
            ico.key().as_ref(), 
            &amount.to_le_bytes()
        ],
        &address_tree_info
            .get_tree_pubkey(&light_cpi_accounts)
            .map_err(|_| ErrorCode::AccountNotEnoughKeys)?,
        &crate::ID,
    );
    
    let new_address_params = address_tree_info.into_new_address_params_packed(address_seed);
    
    // Create the shielded investment account
    let mut investment = light_sdk::account::LightAccount::<ShieldedInvestment>::new_init(
        &crate::ID,
        Some(address),
        output_merkle_tree_index,
    );
    
    // Populate the investment data
    investment.account.investor = ctx.accounts.investor.key().to_bytes();
    investment.account.amount = amount;
    investment.account.ico = ico.key().to_bytes();
    investment.account.timestamp = now;
    investment.account.investment_id = ico.total_raised / ico.price_lamports;
    
    // Prepare CPI call to Light Protocol
    let cpi = CpiInputs::new_with_address(
        proof,
        vec![investment.to_account_info().map_err(ProgramError::from)?],
        vec![new_address_params],
    );
    
    // Execute the Light Protocol CPI
    cpi.invoke_light_system_program(light_cpi_accounts)
        .map_err(|e| {
            msg!("Light Protocol CPI failed: {:?}", e);
            ProgramError::from(e)
        })?;
    
    // Update ICO state atomically
    ico.total_tokens = ico.total_tokens
        .checked_sub(amount)
        .ok_or(ErrorCode::NotEnoughTokens)?;
    
    ico.total_raised = ico.total_raised
        .checked_add(amount)
        .ok_or(ErrorCode::OverflowError)?;
    
    // Emit investment event (this will be visible on-chain)
    emit!(InvestmentEvent {
        ico: ico.key(),
        investor: ctx.accounts.investor.key(),
        amount,
        timestamp: now,
        total_raised: ico.total_raised,
        remaining_tokens: ico.total_tokens,
    });
    
    Ok(())
}

// Shielded investment structure for Light Protocol
#[derive(Clone, Debug, Default, AnchorSerialize, AnchorDeserialize, LightDiscriminator, LightHasher)]
pub struct ShieldedInvestment {
    #[hash]
    pub investor: [u8; 32], // Use byte array instead of Pubkey for hashing
    pub amount: u64,
    #[hash]
    pub ico: [u8; 32], // Use byte array instead of Pubkey for hashing
    pub timestamp: i64,
    pub investment_id: u64,
}

// Public event for transparency while maintaining investor privacy
#[event]
pub struct InvestmentEvent {
    pub ico: Pubkey,
    pub investor: Pubkey, // This could be omitted for full privacy
    pub amount: u64,
    pub timestamp: i64,
    pub total_raised: u64,
    pub remaining_tokens: u64,
}