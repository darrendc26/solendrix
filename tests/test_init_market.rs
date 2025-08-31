// use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::{ Pubkey},
    signer::Signer, system_program
};

mod basic;
use basic::*;

 #[test]
    fn test_init_market() {
        // 1. Setup VM and program
        let (mut svm, admin, program_id) = setup_svm_and_program();

        // 2. Derive PDA for market account (same seeds your program uses)
        let (market_pda, _bump) = Pubkey::find_program_address(
            &[b"market", admin.pubkey().as_ref()],
            &program_id,
        );

         let fee_vault = Pubkey::new_unique();
        let vault_a = Pubkey::new_unique();
        let vault_b = Pubkey::new_unique();


        // 3. Build instruction
        let ix = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(admin.pubkey(), true),       // admin (signer)
                AccountMeta::new(market_pda, false),          // market account (PDA)
                AccountMeta::new_readonly(fee_vault, false),  // fee_vault
                AccountMeta::new_readonly(vault_a, false),    // vault_a  
                AccountMeta::new_readonly(vault_b, false),    // vault_b
                AccountMeta::new_readonly(system_program::id(), false), // system_program
            ],
            data: vec![
                0, 
                10, 0, 0, 0, 0, 0, 0, 0, // liquidity threshold
                5, 0, 0, 0, 0, 0, 0, 0,  // fee
            ],
        };

        // 4. Send transaction
        let res = build_and_send_transaction(&mut svm, &admin, vec![ix]);

        // 5. Assert success
        assert!(res.is_ok(), "Transaction failed: {:?}", res.err());
    }
