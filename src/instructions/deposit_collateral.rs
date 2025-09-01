use pinocchio::sysvars::Sysvar;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult, 
    pubkey::{find_program_address}};
use pinocchio_token::instructions::Transfer;
use crate::instructions::helpers::*;
use crate::state::{user::User, market::Market};
use pinocchio::sysvars::clock::Clock;
pub struct DepositCollateralAccounts<'a> {
    pub user: &'a AccountInfo,
    pub user_pda: &'a AccountInfo,
    pub market: &'a AccountInfo,
    pub user_token_account: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub vault_a: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositCollateralAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, user_pda, market, user_token_account, mint_a, vault_a, system_program, token_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        SignerAccount::check(user)?;
        ProgramAccount::check(user_pda)?;
        ProgramAccount::check(market)?;
        TokenAccountInterface::check(user_token_account)?;
        MintInterface::check(mint_a)?;
        TokenAccountInterface::check(vault_a)?;

        Ok(Self {
            user: user,
            user_pda: user_pda,
            market: market,
            user_token_account: user_token_account,
            mint: mint_a,
            vault_a: vault_a,
            system_program: system_program,
            token_program: token_program,
        })
    }
}

pub struct DepositCollateralData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DepositCollateralData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<DepositCollateralData>() {
            return Err(ProgramError::InvalidAccountData);
        }
        let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());

        if !amount > 0 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Self { amount })
    }
}        

pub struct DepositCollateral<'a> {
    pub accounts: DepositCollateralAccounts<'a>,
    pub data: DepositCollateralData,

}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for DepositCollateral<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = DepositCollateralAccounts::try_from(accounts)?;
        let data = DepositCollateralData::try_from(data)?;

        let (user_pda, _bump) = find_program_address(
            &[b"user", accounts.user.key().as_ref()],
            &crate::ID, // your program ID
        );
        if user_pda != *accounts.user_pda.key(){
            return Err(ProgramError::InvalidSeeds);
        }

        let (market_pda, _bump) = find_program_address(
            &[b"market", accounts.user.key().as_ref()],
            &crate::ID, // your program ID
        );
        if market_pda != *accounts.market.key(){
            return Err(ProgramError::InvalidSeeds);
        }

        Ok(Self {
            accounts,
            data,
        })
    }
}

impl<'a> DepositCollateral<'a> {
    pub const DISCRIMINATOR: u8 = 2;

    pub fn process(&self) -> ProgramResult {
        let mut data = self.accounts.user_pda.try_borrow_mut_data()?;
        let mut market_data = self.accounts.market.try_borrow_mut_data()?;
        let market = Market::load_mut(&mut market_data)?;
        let user = User::load_mut(&mut data)?;
        let now = Clock::get()?.unix_timestamp; 

        let _ = Transfer {
            from: self.accounts.user_token_account,
            to: self.accounts.vault_a,
            authority: self.accounts.user,
            amount: self.data.amount,
        }
        .invoke();
    
        user.set_total_deposits(user.total_deposits + self.data.amount);
        user.set_last_update_ts(now);

        market.set_total_deposits(market.total_deposits + self.data.amount);

        Ok(())
    }
}