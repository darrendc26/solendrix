use pinocchio::{entrypoint, instruction, pubkey::Pubkey, program_error::ProgramError, account_info::AccountInfo, ProgramResult};
entrypoint!(process_instruction);

pub mod state;

pinocchio_pubkey::declare_id!("Dr2Y39b8JDWbmvug8UPwQvorfsEkyog9r4AR3ZcK5cJU");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);
    match instruction_data.split_first() {
            Some(Deposit) => todo!(),
            _ => Err(ProgramError::InvalidInstructionData),
    }
}