use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, message::Message,
    program_pack::Pack, pubkey::Pubkey, signers::Signers, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

use crate::{client::get_token_account, config::JitoStakePoolCliConfig};

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

fn add_associated_token_account(
    config: &JitoStakePoolCliConfig,
    mint: &Pubkey,
    owner: &Pubkey,
    instructions: &mut Vec<Instruction>,
    rent_free_balances: &mut u64,
) -> Pubkey {
    // Account for tokens not specified, creating one
    let account = get_associated_token_address(owner, mint);
    if get_token_account(&config.rpc_client, &account, mint).is_err() {
        println!("Creating associated token account {} to receive stake pool tokens of mint {}, owned by {}", account, mint, owner);

        let min_account_balance = config
            .rpc_client
            .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)
            .unwrap();

        #[allow(deprecated)]
        instructions.push(create_associated_token_account(
            &config.fee_payer.pubkey(),
            owner,
            mint,
            &spl_token::id(),
        ));

        *rent_free_balances += min_account_balance;
    } else {
        println!("Using existing associated token account {} to receive stake pool tokens of mint {}, owned by {}", account, mint, owner);
    }

    account
}
