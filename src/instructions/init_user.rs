use pinocchio::{account_info::AccountInfo, instruction::Seed,    
            program_error::ProgramError, pubkey::find_program_address,
            sysvars::{clock::Clock, Sysvar}, ProgramResult};
use crate::instructions::helpers::*;
use crate::state::user::User;

pub struct InitUserAccounts<'a> {
    pub user: &'a AccountInfo,
    pub user_pda: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitUserAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, user_pda, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            user: user,
            user_pda: user_pda,
            system_program: system_program,
        })
    }
}

// pub struct InitUserData {}

pub struct InitUser<'a> {
    pub accounts: InitUserAccounts<'a>, 
    pub bump: u8,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitUser<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = InitUserAccounts::try_from(accounts)?;

        let (user_pda, bump) = find_program_address(
            &[b"user", accounts.user.key().as_ref()], 
            &crate::ID);
        if user_pda != *accounts.user_pda.key() {
            return Err(ProgramError::InvalidSeeds);
        }
        let bump_binding = [bump];
        let user_seeds = [
            Seed::from(b"user"),
            Seed::from(accounts.user.key().as_ref()),
            Seed::from(&bump_binding),
        ];

        let _ = ProgramAccount::init::<User>(
            accounts.user,
            accounts.user_pda,
            &user_seeds,
            User::LEN,
        );

        Ok(Self {
            accounts,
            bump,
        })
    }
}

impl<'a> InitUser<'a> {
    pub const DISCRIMINATOR: u8 = 1;

    pub fn process(&self) -> ProgramResult {
        let mut data = self.accounts.user_pda.try_borrow_mut_data()?;
        let user = User::load_mut(&mut data)?;
        let now = Clock::get()?.unix_timestamp;

        user.set_pubkey(*self.accounts.user.key());
        user.set_total_deposits(0);
        user.set_total_borrows(0);
        user.set_last_update_ts(now);
        user.set_bump(self.bump);

        Ok(())
    }
}