
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer, 
    system_program,
};
use spl_token;
// use spl_associated_token_account;
use solendrix::state::User;
mod basic;
use basic::*;

#[test]
fn test_deposit() {
    // 1. Setup VM and program
    let (mut svm, admin, program_id) = setup_svm_and_program();
    

   

    // Use regular token account instead of ATA
    let (mint_pubkey, user_token_account) = setup_mint_and_regular_token_account(
        &mut svm,
        &admin,
        &admin,
        1000,
    );

    let vault = create_vault_token_account(
        &mut svm,
        &admin,
        &mint_pubkey,
        &admin.pubkey(),
    );
 init_market(&mut svm, &admin, program_id, mint_pubkey);
    init_user(&mut svm, &admin, program_id);
    // 2. Derive PDA for market account (same seeds your program uses)
    let (market_pda, _bump) = Pubkey::find_program_address(
        &[b"market", admin.pubkey().as_ref()],
        &program_id,
    );
    
    let (user_pda, _bump) = Pubkey::find_program_address(
        &[b"user", admin.pubkey().as_ref()],
        &program_id,
    );

    // TODO: You need to initialize the market_pda and user_pda accounts first
    // Add initialization instructions here before deposit

    // 3. Build instruction
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(admin.pubkey(), true),                    // user
            AccountMeta::new(user_pda, false),                         // user_pda  
            AccountMeta::new(market_pda, false),                       // market
            AccountMeta::new(user_token_account, false),               // user_token_account
            AccountMeta::new_readonly(mint_pubkey, false),             // mint
            AccountMeta::new(vault, false),                            // vault_a
            AccountMeta::new_readonly(system_program::id(), false),    // system_program
            AccountMeta::new_readonly(spl_token::id(), false),         // token_program
        ],
        data: vec![
            2,                                    // discriminator
            10, 0, 0, 0, 0, 0, 0, 0,             // amount (10 as u64 little-endian)
        ],
    };

    // 4. Send transaction
    let res = build_and_send_transaction(&mut svm, &admin, vec![ix]);

    let account = svm.get_account(&user_pda).unwrap();
    let mut data = account.data.clone();
    let user = User::load_mut(&mut data).unwrap();

    println!("total deposits: {}", user.total_deposits);
    println!("last update ts: {}", user.last_update_ts);

    match res {
        Ok(metadata) => {
            println!("Transaction succeeded!");
            println!("Logs: {:?}", metadata.logs);
        },
        Err(failed_metadata) => {
            println!("Transaction failed: {:?}", failed_metadata.err);
            println!("Logs: {:?}", failed_metadata.meta.logs);
            panic!("Transaction should have succeeded");
        }
    }
}