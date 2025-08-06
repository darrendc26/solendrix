use anchor_lang::prelude::*;
use light_sdk::{
    // account::LightAccount,
    // address::v1::derive_address,
    cpi::{
        // CpiAccounts, CpiInputs, 
        CpiSigner},
    derive_light_cpi_signer,
    instruction::{
        // account_meta::CompressedAccountMeta, 
        PackedAddressTreeInfo, ValidityProof},
    // LightDiscriminator, LightHasher,
};

mod error;
mod ico;
mod initialize_ico;
mod invest;
mod claim;

// Public exports
pub use crate::ico::*;
pub use crate::initialize_ico::*;
pub use crate::invest::*;
pub use claim::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const LIGHT_CPI_SIGNER: CpiSigner =
    derive_light_cpi_signer!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ico_launchpad {
    use super::*;

    /// Initialize a new ICO with the given parameters
    pub fn initialize_ico(
        ctx: Context<InitializeIco>, 
        token_mint: Pubkey, 
        token_account: Pubkey, 
        start_time: i64, 
        end_time: i64, 
        total_tokens: u64, 
        price_lamports: u64
    ) -> Result<()> {
        initialize_ico::initialize_ico_handler(
            ctx, 
            token_mint, 
            token_account, 
            start_time, 
            end_time, 
            total_tokens, 
            price_lamports
        )    
    }

    /// Create a shielded investment in an ICO using Light Protocol
pub fn create_investment<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Invest<'info>>,
        proof: ValidityProof, 
        address_tree_info: PackedAddressTreeInfo, 
        output_merkle_tree_index: u8, 
        amount: u64
    ) -> Result<()> { 
        invest::invest_handler(ctx, proof, address_tree_info, output_merkle_tree_index, amount)
    }
}

// Account structs for additional functions
#[derive(Accounts)]
pub struct UpdateIcoStatus<'info> {
    #[account(mut, has_one = owner)]
    pub ico: Account<'info, Ico>,
    pub owner: Signer<'info>,
}


// Generic accounts struct for Light Protocol operations
#[derive(Accounts)]
pub struct GenericLightAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Light Protocol program
    pub light_protocol_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}