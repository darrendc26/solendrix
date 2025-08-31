use pinocchio::{pubkey::Pubkey, program_error::ProgramError};
use core::mem::size_of;

pub struct User {
    pub pubkey: Pubkey,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub last_update_ts: i64,
    bump: u8,
}

impl User {
    pub const LEN: usize = size_of::<User>();

    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
   }

    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    pub fn set_pubkey(&mut self, pubkey: Pubkey) {
        self.pubkey = pubkey;
    }

    pub fn set_total_deposits(&mut self, total_deposits: u64) {
        self.total_deposits = total_deposits;
    }

    pub fn set_total_borrows(&mut self, total_borrows: u64) {
        self.total_borrows = total_borrows;
    }

    pub fn set_last_update_ts(&mut self, last_update_ts: i64) {
        self.last_update_ts = last_update_ts;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }
}