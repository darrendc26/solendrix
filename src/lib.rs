#![allow(unexpected_cfgs)]
use pinocchio::{entrypoint, pubkey::Pubkey, program_error::ProgramError, account_info::AccountInfo, ProgramResult};

use crate::instructions::{InitMarket, InitUser, DepositCollateral};
entrypoint!(process_instruction);

pub mod state;
pub mod instructions;

pinocchio_pubkey::declare_id!("Dr2Y39b8JDWbmvug8UPwQvorfsEkyog9r4AR3ZcK5cJU");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);
    match instruction_data.split_first() {
        Some((&InitMarket::DISCRIMINATOR, data)) => 
            InitMarket::try_from((data, accounts))?.process(),
        Some((&InitUser::DISCRIMINATOR, _)) => 
            InitUser::try_from(accounts)?.process(),
        Some((&DepositCollateral::DISCRIMINATOR, data)) => 
            DepositCollateral::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
