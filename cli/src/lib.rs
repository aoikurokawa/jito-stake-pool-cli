use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, message::Message,
    signers::Signers, transaction::Transaction,
};

use crate::config::JitoStakePoolCliConfig;

pub mod client;
pub mod command;
pub mod config;

// pub fn get_latest_blockhash(client: &RpcClient) -> Result<Hash, Error> {
//     Ok()
// }

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
