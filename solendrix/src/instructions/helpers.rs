use pinocchio::{
    account_info::AccountInfo,
    ProgramResult,
    program_error::ProgramError,
    // pubkey::{find_program_address, Pubkey},
};
// use pinocchio_associated_token_account::instructions::Create;
use pinocchio::instruction::{Signer, Seed};
use pinocchio_system::instructions::{CreateAccount};
use pinocchio::sysvars::{Sysvar, rent::Rent};

pub trait AccountCheck {
    fn check( account: &AccountInfo ) -> Result<(), ProgramError>;
}

pub struct SignerAccount;

impl AccountCheck for SignerAccount {
    fn check( account: &AccountInfo ) -> Result<(), ProgramError> {
        if account.is_signer() {
            Ok(())
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }
}

pub struct ProgramAccount;

pub trait ProgramAccountCheck {
    fn check( account: &AccountInfo ) -> Result<(), ProgramError>;
}

impl AccountCheck for ProgramAccount {
    fn check( account: &AccountInfo ) -> Result<(), ProgramError> {
        if !account.is_owned_by( &crate::ID ) {
            return Err( ProgramError::IncorrectProgramId );
        }
        Ok(())            
    }
}

pub trait ProgramAccountInit {
    fn init<'a, T: Sized>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult;
}
 
impl ProgramAccountInit for ProgramAccount {
    fn init<'a, T: Sized>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> ProgramResult {
        // Get required lamports for rent
        let lamports = Rent::get()?.minimum_balance(space);
 
        // Create signer with seeds slice
        let signer = [Signer::from(seeds)];
 
        // Create the account
        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: space as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&signer)?;
 
        Ok(())
    }
}
