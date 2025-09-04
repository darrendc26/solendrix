use pinocchio::sysvars::Sysvar;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};
// use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult,
//         pubkey::{find_program_address}};
use crate::instructions::helpers::*;
use crate::state::{market::Market, user::User};
use pinocchio::sysvars::clock::Clock;
use pinocchio_token::instructions::Transfer;

pub struct BorrowAccounts<'a> {
    pub user: &'a AccountInfo,
    pub admin: &'a AccountInfo,
    pub user_pda: &'a AccountInfo,
    pub market: &'a AccountInfo,
    pub user_token_account_b: &'a AccountInfo,
    pub mint_b: &'a AccountInfo,
    pub vault_b: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for BorrowAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, admin, user_pda, market, user_token_account_b, mint_b, vault_b, system_program, token_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        SignerAccount::check(user)?;
        SignerAccount::check(admin)?;
        ProgramAccount::check(user_pda)?;
        ProgramAccount::check(market)?;
        TokenAccountInterface::check(user_token_account_b)?;
        MintInterface::check(mint_b)?;
        TokenAccountInterface::check(vault_b)?;

        Ok(Self {
            user: user,
            admin: admin,
            user_pda: user_pda,
            market: market,
            user_token_account_b: user_token_account_b,
            mint_b: mint_b,
            vault_b: vault_b,
            system_program: system_program,
            token_program: token_program,
        })
    }
}

pub struct BorrowData {
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for BorrowData {
    type Error = ProgramError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < size_of::<u8>() {
            return Err(ProgramError::InvalidAccountData);
        }
        let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
        if amount == 0 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Self { amount })
    }
}

pub struct Borrow<'a> {
    pub accounts: BorrowAccounts<'a>,
    pub data: BorrowData,
    pub market_bump: u8,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Borrow<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = BorrowAccounts::try_from(accounts)?;
        let data = BorrowData::try_from(data)?;

        let (user_pda, _bump) = find_program_address(
            &[b"user", accounts.user.key().as_ref()],
            &crate::ID, // your program ID
        );
        if user_pda != *accounts.user_pda.key() {
            return Err(ProgramError::InvalidSeeds);
        }

        let (market_pda, market_bump) = find_program_address(
            &[b"market", accounts.user.key().as_ref()],
            &crate::ID, // your program ID
        );
        if market_pda != *accounts.market.key() {
            return Err(ProgramError::InvalidSeeds);
        }

        Ok(Self {
            accounts,
            data,
            market_bump,
        })
    }
}

impl<'a> Borrow<'a> {
    pub const DISCRIMINATOR: u8 = 3;

    pub fn process(&self) -> ProgramResult {
        let mut user_data = self.accounts.user_pda.try_borrow_mut_data()?;
        let mut market_data = self.accounts.market.try_borrow_mut_data()?;
        let user = User::load_mut(&mut user_data)?;
        let market = Market::load_mut(&mut market_data)?;
        let now = {
            let clock = Clock::get()?;
            clock.unix_timestamp
        };

        let amount = self.data.amount;

        if amount > user.total_deposits {
            return Err(ProgramError::InsufficientFunds);
        }

        let fee = amount * market.fee / 10000;
        let amount = amount - fee;

        let seed_binding = [self.market_bump];
        let market_seeds = [
            Seed::from(b"market"),
            Seed::from(self.accounts.admin.key().as_ref()),
            Seed::from(&seed_binding),
        ];
        let signer = [Signer::from(&market_seeds)];
        Transfer {
            from: self.accounts.vault_b,
            to: self.accounts.user_token_account_b,
            authority: self.accounts.market,
            amount,
        }
        .invoke_signed(&signer)?;

        user.set_total_deposits(user.total_deposits - amount);
        user.set_last_update_ts(now);
        user.set_total_borrows(user.total_borrows + amount);

        market.set_total_deposits(market.total_deposits - amount);
        market.set_total_borrows(market.total_borrows + amount);

        Ok(())
    }
}

// TODO: Have to send fees to fee vault
