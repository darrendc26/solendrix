
use solana_sdk::{
    instruction::{ AccountMeta, Instruction}, pubkey::Pubkey, signer::Signer, system_program
};

mod basic;
use basic::*;

#[test]
fn test_init_user() {
    let (mut svm, user, program_id) = setup_svm_and_program();
    
    let (user_pda, _bump) = Pubkey::find_program_address(
        &[b"user", user.pubkey().as_ref()],
        &program_id,
    );

    let ix = Instruction {
            program_id,
            accounts: vec![        
                AccountMeta::new(user.pubkey(), true),       // user (signer)
                AccountMeta::new(user_pda, false),          // user_pda  
                AccountMeta::new_readonly(system_program::id(), false), // system_program
            ],
            data: vec![
                1, 
            ],
            };

    let res = build_and_send_transaction(&mut svm, &user, vec![ix]);
    assert!(res.is_ok(), "Transaction failed: {:?}", res.err());
    
}