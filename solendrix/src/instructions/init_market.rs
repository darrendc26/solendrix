use pinocchio::{account_info::AccountInfo, ProgramResult,
            instruction::{Account, Instruction, Seed, Signer}, 
            program_error::ProgramError, pubkey::{checked_create_program_address, 
                find_program_address, Pubkey}};
use crate::{state::market::Market, ID};
use crate::instructions::helpers::*;
use crate::instructions::{ AccountCheck};


pub struct InitMarketAccounts<'a> {
    pub admin: &'a AccountInfo,
    pub market: &'a AccountInfo,
    pub fee_vault: &'a AccountInfo,
    pub vault_a: &'a AccountInfo,
    pub vault_b: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}


impl<'a> TryFrom< &'a [AccountInfo] > for InitMarketAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [admin, market,fee_vault, vault_a, vault_b, system_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        SignerAccount::check(admin)?;

        Ok(Self {
            admin,
            market,
            fee_vault,
            vault_a,
            vault_b,
            system_program,
        })
    }
}

pub struct InitMarketData {
    pub liquidity_threshold: u64,
    pub fee: u64,
}

impl<'a> TryFrom< &'a [u8]> for InitMarketData {
    type Error = ProgramError;

    fn try_from((data): &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<InitMarketData>() {
            return Err(ProgramError::InvalidAccountData);
        }
        let liquidity_threshold = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let fee = u64::from_le_bytes(data[8..16].try_into().unwrap());
        
        Ok(Self { liquidity_threshold, fee })
    }
}


pub struct InitMarket<'a> {
    pub accounts: InitMarketAccounts<'a>,
    pub data: InitMarketData,
    pub bump: u8,
}

impl<'a> TryFrom< (&'a [u8], &'a [AccountInfo])> for InitMarket<'a> {
    type Error = ProgramError;

    fn try_from((init_data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = InitMarketAccounts::try_from(accounts)?;
        let init_data = InitMarketData::try_from(init_data)?;
        
        let (market_key, bump) = find_program_address(
            &[b"market", accounts.admin.key().as_ref()],
            &crate::ID, // your program ID
        );

        if market_key != *accounts.market.key(){
            return Err(ProgramError::InvalidSeeds);
        }

        let bump_binding = [bump];

        let market_seeds = [
            Seed::from(b"market"),
            Seed::from(accounts.admin.key().as_ref()),
            Seed::from(&bump_binding),
        ];

        ProgramAccount::init::<Market>(
            accounts.admin,
            accounts.market,
            &market_seeds,
            Market::LEN,
        );

        Ok(Self {
            accounts,
            data: init_data,
            bump,
        })
    }
}

impl<'a> InitMarket<'a> {
    pub const DISCRIMINATOR: u8 = 0;

    pub fn process(&self) -> ProgramResult {
        // let accounts = self.accounts;
        // let init_data = self.data;
        let bump = self.bump;
    let mut data = self.accounts.market.try_borrow_mut_data()?;
    let market = Market::load_mut(&mut data)?;
    market.set_admin(*self.accounts.admin.key());
    market.set_total_deposits(0);
    market.set_total_withdrawals(0);
    market.set_liquidity_threshold(self.data.liquidity_threshold);
    market.set_fee(self.data.fee);
    market.set_fee_vault(*self.accounts.fee_vault.key());
    market.set_vault_a(*self.accounts.vault_a.key());
    market.set_vault_b(*self.accounts.vault_b.key());
    market.set_is_active(true);
    market.set_bump(bump);

        Ok(())
    }
}