use std::str::FromStr;

use clap::Args;
use jito_stake_pool_sdk::sdk::add_validator::add_validator_to_pool_with_vote;
use solana_sdk::pubkey::Pubkey;

use crate::{
    checked_transaction_with_signers,
    client::{get_stake_pool, get_validator_list},
    config::JitoStakePoolCliConfig,
    send_transaction,
};

#[derive(Args)]
pub struct AddValidatorArgs {
    /// Stake pool address
    pub pool: String,

    /// The validator vote account that the stake is delegated to
    pub vote_account: String,
}

pub fn command_vsa_add(
    config: &JitoStakePoolCliConfig,
    stake_pool_address: &Pubkey,
    vote_account_address: &Pubkey,
) -> anyhow::Result<()> {
    let vote_account =
        solana_program::pubkey::Pubkey::new_from_array(vote_account_address.to_bytes());
    // let (stake_account_address, _) =
    //     find_stake_program_address(&spl_stake_pool::id(), vote_account, stake_pool_address);

    let program_id = Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

    let stake_account_address = Pubkey::find_program_address(
        &[&vote_account.to_bytes(), &stake_pool_address.to_bytes()],
        &program_id,
    )
    .0;

    println!("Adding stake account {stake_account_address}, delegated to {vote_account}",);

    let stake_pool = get_stake_pool(&config.rpc_client, stake_pool_address)?;
    let validator_list = Pubkey::new_from_array(stake_pool.validator_list.to_bytes());

    let validator_list = get_validator_list(&config.rpc_client, &validator_list)?;

    if validator_list.contains(vote_account_address) {
        eprintln!("Stake pool already contains validator {vote_account}, ignoring",);
        return Ok(());
    }

    //     if !config.no_update {
    //         command_update(config, stake_pool_address, false, false)?;
    //     }
    //
    let signers = vec![config.fee_payer.as_ref(), config.staker.as_ref()];

    let fee_payer = Pubkey::new_from_array(config.fee_payer.pubkey().to_bytes());

    // unique_signers!(signers);
    let transaction = checked_transaction_with_signers(
        config,
        &[add_validator_to_pool_with_vote(
            &spl_stake_pool::id(),
            &stake_pool,
            stake_pool_address,
            &fee_payer,
            vote_account_address,
        )],
        &signers,
    )?;

    send_transaction(config, transaction)?;

    Ok(())
}
