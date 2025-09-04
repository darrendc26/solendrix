use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    system_program,
};
use solendrix::state::User;
use spl_token;
mod basic;
use basic::*;

#[test]
fn test_withdraw() {
    let (mut svm, admin, user, program_id) = setup_svm_and_program();

    let (mint_pubkey, user_token_account) =
        setup_mint_and_regular_token_account(&mut svm, &user, &user, 1000);

    init_market(&mut svm, &admin, program_id, mint_pubkey);
    init_user(&mut svm, &user, program_id);

    let (market_pda, _bump) =
        Pubkey::find_program_address(&[b"market", admin.pubkey().as_ref()], &program_id);

    let (user_pda, _bump) =
        Pubkey::find_program_address(&[b"user", user.pubkey().as_ref()], &program_id);

    let vault = create_vault_token_account(&mut svm, &admin, &mint_pubkey, &market_pda);

    let dep_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user.pubkey(), true),                  // user
            AccountMeta::new(admin.pubkey(), false),                // admin
            AccountMeta::new(market_pda, false),                    // market
            AccountMeta::new(user_pda, false),                      // user_pda
            AccountMeta::new(user_token_account, false),            // user_token_account
            AccountMeta::new_readonly(mint_pubkey, false),          // mint
            AccountMeta::new(vault, false),                         // vault_a
            AccountMeta::new_readonly(system_program::id(), false), // system_program
            AccountMeta::new_readonly(spl_token::id(), false),      // token_program
        ],
        data: vec![
            2, // discriminator
            10, 0, 0, 0, 0, 0, 0, 0, // amount
        ],
    };

    let dep_res = build_and_send_transaction(&mut svm, &user, vec![dep_ix]);

    println!("Deposit result:");
    match dep_res {
        Ok(metadata) => {
            println!("Transaction succeeded!");
            println!("Logs: {:?}", metadata.logs);
        }
        Err(failed_metadata) => {
            println!("Transaction failed: {:?}", failed_metadata.err);
            println!("Logs: {:?}", failed_metadata.meta.logs);
            panic!("Transaction should have succeeded");
        }
    }

    let account = svm.get_account(&user_pda).unwrap();
    let mut data = account.data.clone();
    let user_account = User::load_mut(&mut data).unwrap();

    println!("total deposits: {}", user_account.total_deposits);
    println!("last update ts: {}", user_account.last_update_ts);

    let withdraw_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user.pubkey(), true),                  // user
            AccountMeta::new(admin.pubkey(), false),                // admin
            AccountMeta::new(user_pda, false),                      // user_pda
            AccountMeta::new(market_pda, false),                    // market
            AccountMeta::new(user_token_account, false),            // user_token_account
            AccountMeta::new_readonly(mint_pubkey, false),          // mint
            AccountMeta::new(vault, false),                         // vault_a
            AccountMeta::new_readonly(system_program::id(), false), // system_program
            AccountMeta::new_readonly(spl_token::id(), false),      // token_program
        ],
        data: vec![
            4, // discriminator
            8, 0, 0, 0, 0, 0, 0, 0, // amount
        ],
    };

    let res = build_and_send_transaction(&mut svm, &user, vec![withdraw_ix]);

    match res {
        Ok(metadata) => {
            println!("Transaction succeeded!");
            println!("Logs: {:?}", metadata.logs);
        }
        Err(failed_metadata) => {
            println!("Transaction failed: {:?}", failed_metadata.err);
            println!("Logs: {:?}", failed_metadata.meta.logs);
            panic!("Transaction should have succeeded");
        }
    }

    let account = svm.get_account(&user_pda).unwrap();
    let mut data = account.data.clone();
    let user_account = User::load_mut(&mut data).unwrap();

    println!("total deposits: {}", user_account.total_deposits);
    println!("last update ts: {}", user_account.last_update_ts);
}
