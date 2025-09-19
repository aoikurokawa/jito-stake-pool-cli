use anyhow::anyhow;
use clap::Args;
use jito_stake_pool_sdk::sdk::increase_validator_stake::increase_validator_stake_with_vote;
use solana_sdk::{native_token, pubkey::Pubkey};

use crate::{
    checked_transaction_with_signers,
    client::{get_stake_pool, get_validator_list},
    config::JitoStakePoolCliConfig,
    send_transaction,
};

#[derive(Args)]
pub struct IncreaseValidatorStakeArgs {
    /// Stake pool address
    pub pool: String,

    /// Vote account for the validator to increase stake to
    pub vote_account: String,

    /// Amount in SOL to add to the validator stake account. Must be at least the rent-exempt amount for a stake plus 1 SOL for merging.
    pub amount: Option<f64>,
}

pub fn command_increase_validator_stake(
    config: &JitoStakePoolCliConfig,
    stake_pool_address: &Pubkey,
    vote_account: &Pubkey,
    amount: f64,
) -> anyhow::Result<()> {
    let lamports = native_token::sol_to_lamports(amount);
    // if !config.no_update {
    //     command_update(config, stake_pool_address, false, false)?;
    // }

    let stake_pool = get_stake_pool(&config.rpc_client, stake_pool_address)?;
    let validator_list = get_validator_list(&config.rpc_client, &stake_pool.validator_list)?;
    let validator_stake_info = validator_list
        .find(vote_account)
        .ok_or(anyhow!("Vote account not found in validator list"))?;

    let mut signers = vec![config.fee_payer.as_ref(), config.staker.as_ref()];
    // unique_signers!(signers);
    let seed: u64 = validator_stake_info.transient_seed_suffix.into();
    let transaction = checked_transaction_with_signers(
        config,
        &[increase_validator_stake_with_vote(
            &spl_stake_pool::id(),
            &stake_pool,
            stake_pool_address,
            vote_account,
            lamports,
            seed,
        )],
        &signers,
    )?;

    send_transaction(config, transaction)?;

    Ok(())
}
