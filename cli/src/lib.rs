use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, message::Message,
    signers::Signers, transaction::Transaction,
};

use crate::config::JitoStakePoolCliConfig;

pub mod client;
pub mod command;
pub mod config;

pub fn send_transaction(
    config: &JitoStakePoolCliConfig,
    transaction: Transaction,
) -> anyhow::Result<()> {
    if config.dry_run {
        let result = config.rpc_client.simulate_transaction(&transaction)?;
        println!("Simulate result: {result:?}");
    } else {
        let signature = config
            .rpc_client
            .send_and_confirm_transaction_with_spinner(&transaction)?;
        println!("Signature: {signature}");
    }
    Ok(())
}

pub fn checked_transaction_with_signers<T: Signers>(
    config: &JitoStakePoolCliConfig,
    instructions: &[Instruction],
    signers: &T,
) -> anyhow::Result<Transaction> {
    let recent_blockhash = config
        .rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())?
        .0;
    // let recent_blockhash = get_latest_blockhash(&config.rpc_client)?;
    let message = Message::new_with_blockhash(
        instructions,
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );
    // check_fee_payer_balance(config, config.rpc_client.get_fee_for_message(&message)?)?;
    let transaction = Transaction::new(signers, message, recent_blockhash);
    Ok(transaction)
}
