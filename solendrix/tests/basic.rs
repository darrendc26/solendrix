use litesvm::{
    types::{FailedTransactionMetadata, TransactionMetadata},
    LiteSVM,
};
use solana_sdk::{
    instruction::{ Instruction},
    message::{v0, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    
    transaction::{ VersionedTransaction},
};
use solendrix::ID;

pub fn setup_svm_and_program() -> (LiteSVM, Keypair, Pubkey) {
    let mut svm = LiteSVM::new();
    let fee_payer = Keypair::new();

    svm.airdrop(&fee_payer.pubkey(), 100000000).unwrap();

    let program_id = Pubkey::from(ID);
    svm.add_program_from_file(program_id, "./target/deploy/solendrix.so").unwrap();

    (svm, fee_payer, program_id)
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