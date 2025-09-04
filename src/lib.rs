#![allow(unexpected_cfgs)]
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::instructions::{Borrow, DepositCollateral, InitMarket, InitUser, WithdrawCollateral};
entrypoint!(process_instruction);

pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("Dr2Y39b8JDWbmvug8UPwQvorfsEkyog9r4AR3ZcK5cJU");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);
    match instruction_data.split_first() {
        Some((&InitMarket::DISCRIMINATOR, data)) => {
            InitMarket::try_from((data, accounts))?.process()
        }
        Some((&InitUser::DISCRIMINATOR, _)) => InitUser::try_from(accounts)?.process(),
        Some((&DepositCollateral::DISCRIMINATOR, data)) => {
            DepositCollateral::try_from((data, accounts))?.process()
        }
        Some((&Borrow::DISCRIMINATOR, data)) => Borrow::try_from((data, accounts))?.process(),
        Some((&WithdrawCollateral::DISCRIMINATOR, data)) => {
            WithdrawCollateral::try_from((data, accounts))?.process()
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
