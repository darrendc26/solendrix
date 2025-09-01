use core::mem::size_of;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

pub struct Market {
    pub admin: Pubkey,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub liquidity_threshold: u64,
    pub fee: u64,
    pub fee_vault: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub is_active: bool,
    pub bump: u8,
}

impl Market {
    pub const LEN: usize = size_of::<Market>();

    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Market::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != Market::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    pub fn set_admin(&mut self, admin: Pubkey) {
        self.admin = admin;
    }

    pub fn set_total_deposits(&mut self, total_deposits: u64) {
        self.total_deposits = total_deposits;
    }

    pub fn set_total_borrows(&mut self, total_borrows: u64) {
        self.total_borrows = total_borrows;
    }

    pub fn set_liquidity_threshold(&mut self, liquidity_threshold: u64) {
        self.liquidity_threshold = liquidity_threshold;
    }

    pub fn set_fee(&mut self, fee: u64) {
        self.fee = fee;
    }

    pub fn set_fee_vault(&mut self, fee_vault: Pubkey) {
        self.fee_vault = fee_vault;
    }

    pub fn set_vault_a(&mut self, vault_a: Pubkey) {
        self.vault_a = vault_a;
    }

    pub fn set_vault_b(&mut self, vault_b: Pubkey) {
        self.vault_b = vault_b;
    }

    pub fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn set_bump(&mut self, bump: u8) {
        self.bump = bump;
    }
}
