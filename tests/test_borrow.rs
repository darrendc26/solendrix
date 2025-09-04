// use solana_program::{instruction::Instruction, pubkey::Pubkey};
// use solana_sdk::{
//     account::Account,
//     instruction::{AccountMeta, Instruction as SdkInstruction},
//     signature::{Keypair, Signer},
//     transaction::Transaction,
// };

// use solendrix::{
//     instructions::{Borrow, InitMarket, InitUser},
//     state::{Market, User},
// };

// #[test]
// fn test_borrow() {
//     let (mut svm, admin, program_id) = setup_svm_and_program();

//     let (market_pda, _bump) =
//         Pubkey::find_program_address(&[b"market", admin.pubkey().as_ref()], &program_id);

//     let (user_pda, _bump) =
//         Pubkey::find_program_address(&[b"user", admin.pubkey().as_ref()], &program_id);

//     let (mint_pubkey, user_token_account) =
//         setup_mint_and_regular_token_account(&mut svm, &admin, &admin, 1000);

//     let vault = create_vault_token_account(&mut svm, &admin, &mint_pubkey, &admin.pubkey());

//     init_market(&mut svm, &admin, program_id, mint_pubkey);
//     init_user(&mut svm, &admin, program_id);

//     let (user_pda, _bump) =
//         Pubkey::find_program_address(&[b"user", admin.pubkey().as_ref()], &program_id);
// }
