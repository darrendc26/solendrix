use pinocchio::{
    account_info::AccountInfo,
    ProgramResult,
    program_error::ProgramError,
    pubkey::{find_program_address},
};
// use pinocchio_associated_token_account::instructions::Create;
use pinocchio::instruction::{Signer, Seed};
use pinocchio_system::instructions::{CreateAccount};
use pinocchio::sysvars::{Sysvar, rent::Rent};

// TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
pub const TOKEN_2022_PROGRAM_ID: [u8; 32] = [
    0x06, 0xdd, 0xf6, 0xe1, 0xee, 0x75, 0x8f, 0xde, 0x18, 0x42, 0x5d, 0xbc, 0xe4, 0x6c, 0xcd, 0xda,
    0xb6, 0x1a, 0xfc, 0x4d, 0x83, 0xb9, 0x0d, 0x27, 0xfe, 0xbd, 0xf9, 0x28, 0xd8, 0xa1, 0x8b, 0xfc,
];
 
const TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET: usize = 165;
pub const TOKEN_2022_MINT_DISCRIMINATOR: u8 = 0x01;
pub const TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 0x02;

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

pub struct TokenAccountInterface;
 
impl AccountCheck for TokenAccountInterface {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&TOKEN_2022_PROGRAM_ID) {
            if !account.is_owned_by(&pinocchio_token::ID) {
                return Err(ProgramError::InvalidAccountOwner.into());
            } else {
                if account.data_len().ne(&pinocchio_token::state::TokenAccount::LEN) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        } else {
            let data = account.try_borrow_data()?;
 
            if data.len().ne(&pinocchio_token::state::TokenAccount::LEN) {
                if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
                if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET]
                    .ne(&TOKEN_2022_TOKEN_ACCOUNT_DISCRIMINATOR)
                {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        }
 
        Ok(())
    }
}

pub struct AssociatedTokenAccount;

pub trait AssociatedTokenAccountCheck {
    fn check(account: &AccountInfo, authority: &AccountInfo,
        mint: &AccountInfo, token_program: &AccountInfo
    ) -> Result<(), ProgramError>;
}


impl AssociatedTokenAccountCheck for AssociatedTokenAccount {
    fn check(account: &AccountInfo, authority: &AccountInfo,
        mint: &AccountInfo, token_program: &AccountInfo
    ) -> Result<(), ProgramError> {
        TokenAccountInterface::check(account)?;

        if find_program_address(&[authority.key(), token_program.key(), mint.key()], &pinocchio_associated_token_account::ID)
        .0.ne(account.key()) {
            return Err(ProgramError::InvalidSeeds.into());
        }
        Ok(())
    }
}

pub struct MintInterface;
 
impl AccountCheck for MintInterface {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&TOKEN_2022_PROGRAM_ID) {
            if !account.is_owned_by(&pinocchio_token::ID) {
                return Err(ProgramError::InvalidAccountOwner.into());
            } else {
                if account.data_len().ne(&pinocchio_token::state::Mint::LEN) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        } else {
            let data = account.try_borrow_data()?;
 
            if data.len().ne(&pinocchio_token::state::Mint::LEN) {
                if data.len().le(&TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
                if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET].ne(&TOKEN_2022_MINT_DISCRIMINATOR) {
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
        }
 
        Ok(())
    }
}