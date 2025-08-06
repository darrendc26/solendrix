// use anchor_lang::prelude::*;
// use light_sdk::{
//     cpi::{CpiAccounts, CpiInputs, CpiSigner},
//     instruction::{PackedAddressTreeInfo, ValidityProof},
//     derive_light_cpi_signer,
//     LightDiscriminator,
//     LightHasher,
// };
// use anchor_lang::solana_program;
// use anchor_spl::token::{TokenAccount, Mint, Token, Transfer, transfer};
// use anchor_spl::associated_token::AssociatedToken;
// use crate::error::ErrorCode;
// use crate::ico::*;

// pub const LIGHT_CPI_SIGNER: CpiSigner =
//     derive_light_cpi_signer!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// pub const LIGHT_PROTOCOL_ID: Pubkey = solana_program::pubkey!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[derive(Accounts)]
// pub struct Claim<'info> {
//     #[account(mut, has_one = owner)]
//     pub ico: Account<'info, Ico>,
    
//     #[account(mut)]
//     pub owner: Signer<'info>, // Owner of the ICO (needs to sign)
    
//     #[account(mut)] // Investor claiming tokens
//     pub investor: Signer<'info>,
    
//     // Investor's associated token account
//     #[account(
//         init_if_needed,
//         payer = investor,
//         associated_token::mint = token_mint,
//         associated_token::authority = investor,
//     )]
//     pub investor_ata: Account<'info, TokenAccount>,
    
//     // ICO's token vault
//     #[account(
//         mut,
//         constraint = ico_vault.owner == ico.key(),
//         constraint = ico_vault.mint == ico.token_mint
//     )]
//     pub ico_vault: Account<'info, TokenAccount>,
    
//     pub token_mint: Account<'info, Mint>,
    
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
// }

// pub fn claim_handler<'info>(
//     ctx: Context<'_, '_, '_, 'info, Claim<'info>>,
//     proof: ValidityProof,
//     address_tree_info: PackedAddressTreeInfo,
//     output_merkle_tree_index: u8,
//     amount: u64,
// ) -> Result<()> {
//     let ico = &mut ctx.accounts.ico;
//     Ok(())
// }