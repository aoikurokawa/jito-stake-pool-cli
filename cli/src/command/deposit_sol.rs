use anyhow::anyhow;
use clap::Args;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::Message,
    native_token::{self, Sol},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use spl_stake_pool::find_withdraw_authority_program_address;

use crate::{
    add_associated_token_account, client::get_stake_pool, config::JitoStakePoolCliConfig,
    send_transaction,
};

#[derive(Args)]
pub struct DepositSolArgs {
    /// Stake pool address
    pub pool: String,

    /// Amount in SOL to deposit into the stake pool reserve account.
    pub amount: Option<f64>,

    /// Source account of funds. [default: cli config keypair]
    #[arg(long, value_name = "KEYPAIR")]
    pub from: Option<String>,

    /// Account to receive the minted pool tokens. Defaults to the token-owner's associated pool token account. Creates the account if it does not exist.
    #[arg(long = "token-receiver", value_name = "POOL_TOKEN_RECEIVER_ADDRESS")]
    pub token_receiver: Option<String>,
    /// Account to receive the referral fees for deposits. Defaults to the token receiver.
    #[arg(long, value_name = "REFERRER_TOKEN_ADDRESS")]
    pub referrer: Option<String>,
}

pub fn command_deposit_sol(
    config: &JitoStakePoolCliConfig,
    stake_pool_address: &Pubkey,
    from: &Option<Keypair>,
    pool_token_receiver_account: &Option<Pubkey>,
    referrer_token_account: &Option<Pubkey>,
    amount: f64,
) -> anyhow::Result<()> {
    // if !config.no_update {
    //     command_update(config, stake_pool_address, false, false)?;
    // }

    let amount = native_token::sol_str_to_lamports(&amount.to_string()).unwrap();

    // Check withdraw_from balance
    let from_pubkey = from
        .as_ref()
        .map_or_else(|| config.fee_payer.pubkey(), |keypair| keypair.pubkey());
    let from_balance = config.rpc_client.get_balance(&from_pubkey)?;
    if from_balance < amount {
        return Err(anyhow!(
            "Not enough SOL to deposit into pool: {}.\nMaximum deposit amount is {} SOL.",
            Sol(amount),
            Sol(from_balance)
        ));
    }

    let stake_pool = get_stake_pool(&config.rpc_client, stake_pool_address)?;

    let mut instructions: Vec<Instruction> = vec![];

    // ephemeral SOL account just to do the transfer
    let user_sol_transfer = Keypair::new();
    let mut signers = vec![config.fee_payer.as_ref(), &user_sol_transfer];
    if let Some(keypair) = from.as_ref() {
        signers.push(keypair)
    }

    let mut total_rent_free_balances: u64 = 0;

    // Create the ephemeral SOL account
    instructions.push(solana_system_interface::instruction::transfer(
        &from_pubkey,
        &user_sol_transfer.pubkey(),
        amount,
    ));

    // Create token account if not specified
    let pool_token_receiver_account =
        pool_token_receiver_account.unwrap_or(add_associated_token_account(
            config,
            &stake_pool.pool_mint,
            &config.token_owner.pubkey(),
            &mut instructions,
            &mut total_rent_free_balances,
        ));

    let referrer_token_account = referrer_token_account.unwrap_or(pool_token_receiver_account);

    let pool_withdraw_authority =
        find_withdraw_authority_program_address(&spl_stake_pool::id(), stake_pool_address).0;

    let deposit_instruction = if let Some(deposit_authority) = config.funding_authority.as_ref() {
        let expected_sol_deposit_authority = stake_pool.sol_deposit_authority.ok_or_else(|| {
            anyhow!("SOL deposit authority specified in arguments but stake pool has none")
        })?;
        signers.push(deposit_authority.as_ref());
        if deposit_authority.pubkey() != expected_sol_deposit_authority {
            let error = format!(
                "Invalid deposit authority specified, expected {}, received {}",
                expected_sol_deposit_authority,
                deposit_authority.pubkey()
            );
            return Err(anyhow!("{error}"));
        }

        spl_stake_pool::instruction::deposit_sol_with_authority(
            &spl_stake_pool::id(),
            stake_pool_address,
            &deposit_authority.pubkey(),
            &pool_withdraw_authority,
            &stake_pool.reserve_stake,
            &user_sol_transfer.pubkey(),
            &pool_token_receiver_account,
            &stake_pool.manager_fee_account,
            &referrer_token_account,
            &stake_pool.pool_mint,
            &spl_token::id(),
            amount,
        )
    } else {
        spl_stake_pool::instruction::deposit_sol(
            &spl_stake_pool::id(),
            stake_pool_address,
            &pool_withdraw_authority,
            &stake_pool.reserve_stake,
            &user_sol_transfer.pubkey(),
            &pool_token_receiver_account,
            &stake_pool.manager_fee_account,
            &referrer_token_account,
            &stake_pool.pool_mint,
            &spl_token::id(),
            amount,
        )
    };

    instructions.push(deposit_instruction);

    let recent_blockhash = config
        .rpc_client
        .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())?
        .0;
    let message = Message::new_with_blockhash(
        &instructions,
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );
    // check_fee_payer_balance(
    //     config,
    //     total_rent_free_balances + config.rpc_client.get_fee_for_message(&message)?,
    // )?;
    // unique_signers!(signers);
    let transaction = Transaction::new(&signers, message, recent_blockhash);

    send_transaction(config, transaction)?;

    Ok(())
}
