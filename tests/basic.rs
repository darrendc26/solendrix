use litesvm::{
    types::{FailedTransactionMetadata, TransactionMetadata},
    LiteSVM,
};
use solana_sdk::{
    instruction::AccountMeta,
    instruction::Instruction,
    message::{v0, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_program,
    transaction::{Transaction, VersionedTransaction},
};
use spl_token::{instruction as token_instruction, ID as TOKEN_PROGRAM_ID};

use solendrix::ID;

pub fn setup_svm_and_program() -> (LiteSVM, Keypair, Keypair, Pubkey) {
    let mut svm = LiteSVM::new();
    let fee_payer = Keypair::new();
    let user2 = Keypair::new();

    svm.airdrop(&fee_payer.pubkey(), 100000000).unwrap();
    svm.airdrop(&user2.pubkey(), 100000000).unwrap();

    let program_id = Pubkey::from(ID);
    svm.add_program_from_file(program_id, "./target/deploy/solendrix.so")
        .unwrap();

    (svm, fee_payer, user2, program_id)
}

pub fn build_and_send_transaction(
    svm: &mut LiteSVM,
    fee_payer: &Keypair,
    instruction: Vec<Instruction>,
) -> Result<TransactionMetadata, FailedTransactionMetadata> {
    let msg = v0::Message::try_compile(
        &fee_payer.pubkey(),
        &instruction,
        &[],
        svm.latest_blockhash(),
    )
    .unwrap();

    let tx = VersionedTransaction::try_new(VersionedMessage::V0(msg), &[&fee_payer]).unwrap();

    svm.send_transaction(tx)
}

// pub fn setup_mint_and_token_account(
//         svm: &mut LiteSVM,
//         mint_authority: &Keypair,
//         owner: &Keypair,
//         initial_supply: u64,
//     ) -> (Pubkey, Pubkey) {
//         let mint = Keypair::new();
//         let mint_pubkey = mint.pubkey();

//         const MINT_SIZE: usize = 82;
//         // Create mint account
//         let mint_rent = svm.minimum_balance_for_rent_exemption(MINT_SIZE);
//         // let mint_rent = rent.minimum_balance(Mint::LEN);

//         let create_mint_account_ix = solana_sdk::system_instruction::create_account(
//             &mint_authority.pubkey(),
//             &mint_pubkey,
//             mint_rent,
//             MINT_SIZE as u64,
//             &ASSOCIATED_TOKEN_PROGRAM_ID,
//         );

//         let initialize_mint_ix = token_instruction::initialize_mint(
//             &TOKEN_PROGRAM_ID    ,                // token_program
//             &mint_pubkey,
//             &mint_authority.pubkey(),
//             Some(&mint_authority.pubkey()),
//             6, // decimals
//         ).unwrap();

//         let tx = Transaction::new_signed_with_payer(
//             &[create_mint_account_ix, initialize_mint_ix],
//             Some(&mint_authority.pubkey()),
//             &[mint_authority, &mint],
//             svm.latest_blockhash(),
//         );

//         svm.send_transaction(tx).unwrap();

//         // Create associated token account
//         let user_token_account = spl_associated_token_account::get_associated_token_address(
//             &owner.pubkey(),
//             &mint_pubkey,
//         );

//         let create_ata_ix = ata_instruction::create_associated_token_account(
//             &mint_authority.pubkey(),
//             &owner.pubkey(),
//             &mint_pubkey,
//             &ASSOCIATED_TOKEN_PROGRAM_ID,
//         );

//         let tx = Transaction::new_signed_with_payer(
//             &[create_ata_ix],
//             Some(&mint_authority.pubkey()),
//             &[mint_authority],
//             svm.latest_blockhash(),
//         );

//         svm.send_transaction(tx).unwrap();

//         // Mint tokens to user
//         if initial_supply > 0 {
//             let mint_to_ix = token_instruction::mint_to(
//                 &
//                 ASSOCIATED_TOKEN_PROGRAM_ID,
//                 &mint_pubkey,
//                 &user_token_account,
//                 &mint_authority.pubkey(),
//                 &[],
//                 initial_supply,
//             ).unwrap();

//             let tx = Transaction::new_signed_with_payer(
//                 &[mint_to_ix],
//                 Some(&mint_authority.pubkey()),
//                 &[mint_authority],
//                 svm.latest_blockhash(),
//             );

//             svm.send_transaction(tx).unwrap();
//         }

//         (mint_pubkey, user_token_account)
//     }

#[allow(dead_code)]
pub fn create_vault_token_account(
    svm: &mut LiteSVM,
    payer: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Pubkey {
    const TOKEN_ACCOUNT_SIZE: usize = 165;
    let vault = Keypair::new();
    let vault_pubkey = vault.pubkey();

    let token_account_rent = svm.minimum_balance_for_rent_exemption(TOKEN_ACCOUNT_SIZE);

    let create_vault_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &vault_pubkey,
        token_account_rent,
        TOKEN_ACCOUNT_SIZE as u64,
        &TOKEN_PROGRAM_ID,
    );

    let initialize_vault_ix =
        token_instruction::initialize_account(&TOKEN_PROGRAM_ID, &vault_pubkey, mint, owner)
            .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_vault_ix, initialize_vault_ix],
        Some(&payer.pubkey()),
        &[payer, &vault],
        svm.latest_blockhash(),
    );

    svm.send_transaction(tx).unwrap();

    vault_pubkey
}

#[allow(dead_code)]
pub fn setup_mint_and_regular_token_account(
    svm: &mut LiteSVM,
    mint_authority: &Keypair,
    owner: &Keypair,
    initial_supply: u64,
) -> (Pubkey, Pubkey) {
    let mint = Keypair::new();
    let mint_pubkey = mint.pubkey();

    const MINT_SIZE: usize = 82;
    // Create mint account
    let mint_rent = svm.minimum_balance_for_rent_exemption(MINT_SIZE);

    let create_mint_account_ix = solana_sdk::system_instruction::create_account(
        &mint_authority.pubkey(),
        &mint_pubkey,
        mint_rent,
        MINT_SIZE as u64,
        &TOKEN_PROGRAM_ID,
    );

    let initialize_mint_ix = token_instruction::initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &mint_authority.pubkey(),
        Some(&mint_authority.pubkey()),
        6, // decimals
    )
    .unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, initialize_mint_ix],
        Some(&mint_authority.pubkey()),
        &[mint_authority, &mint],
        svm.latest_blockhash(),
    );

    svm.send_transaction(tx).unwrap();

    // Create regular token account (not ATA)
    let user_token_account =
        create_vault_token_account(svm, mint_authority, &mint_pubkey, &owner.pubkey());

    // Mint tokens to user
    if initial_supply > 0 {
        let mint_to_ix = token_instruction::mint_to(
            &TOKEN_PROGRAM_ID,
            &mint_pubkey,
            &user_token_account,
            &mint_authority.pubkey(),
            &[],
            initial_supply,
        )
        .unwrap();

        let tx = Transaction::new_signed_with_payer(
            &[mint_to_ix],
            Some(&mint_authority.pubkey()),
            &[mint_authority],
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).unwrap();
    }

    (mint_pubkey, user_token_account)
}

#[allow(dead_code)]
pub fn init_user(svm: &mut LiteSVM, user: &Keypair, program_id: Pubkey) {
    let (user_pda, _bump) =
        Pubkey::find_program_address(&[b"user", user.pubkey().as_ref()], &program_id);
    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user.pubkey(), true), // user (signer)
            AccountMeta::new(user_pda, false),     // user_pda
            AccountMeta::new_readonly(system_program::id(), false), // system_program
        ],
        data: vec![1],
    };
    build_and_send_transaction(svm, user, vec![ix]).unwrap();
}
#[allow(dead_code)]
pub fn init_market(svm: &mut LiteSVM, admin: &Keypair, program_id: Pubkey, mint_pubkey: Pubkey) {
    let (market_pda, _bump) =
        Pubkey::find_program_address(&[b"market", admin.pubkey().as_ref()], &program_id);

    let vault_a = create_vault_token_account(svm, admin, &mint_pubkey, &market_pda);
    let vault_b = create_vault_token_account(svm, admin, &mint_pubkey, &market_pda);
    let fee_vault = create_vault_token_account(svm, admin, &mint_pubkey, &market_pda);

    let ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(admin.pubkey(), true),      // admin (signer)
            AccountMeta::new(market_pda, false),         // market account (PDA)
            AccountMeta::new_readonly(fee_vault, false), // fee_vault
            AccountMeta::new_readonly(vault_a, false),   // vault_a
            AccountMeta::new_readonly(vault_b, false),   // vault_b
            AccountMeta::new_readonly(system_program::id(), false), // system_program
        ],
        data: vec![
            0, 10, 0, 0, 0, 0, 0, 0, 0, // liquidity threshold
            5, 0, 0, 0, 0, 0, 0, 0, // fee
        ],
    };

    build_and_send_transaction(svm, admin, vec![ix]).unwrap();
}
