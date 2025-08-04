#![allow(unused)]
use anchor_lang::prelude::*;
use light_sdk::{
    account::LightAccount,
    address::v1::derive_address,
    cpi::{CpiAccounts, CpiInputs, CpiSigner},
    derive_light_cpi_signer,
    instruction::{account_meta::CompressedAccountMeta, PackedAddressTreeInfo, ValidityProof},
    LightDiscriminator, LightHasher,
};

mod error;
mod ico;
mod initialize_ico;
mod invest;
pub use invest::*;
pub use ico::*;
pub use initialize_ico::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const LIGHT_CPI_SIGNER: CpiSigner =
    derive_light_cpi_signer!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ico_launchpad {

    use super::*;

    pub fn initialize_ico(ctx: Context<InitializeIco>, token_mint: Pubkey, token_account: Pubkey, start_time: i64, end_time: i64, total_tokens: u64, price_lamports: u64) -> Result<()> {
        initialize_ico::initialize_ico_handler(ctx, token_mint, token_account, start_time, end_time, total_tokens, price_lamports)    
    }
}

// Declare compressed account as event so that it is included in the anchor idl.
#[event]
#[derive(
    Clone, Debug, Default, LightDiscriminator, LightHasher,
)]
pub struct CounterCompressedAccount {
    #[hash]
    pub owner: Pubkey,
    pub counter: u64,
}

#[derive(Accounts)]
pub struct GenericAnchorAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
