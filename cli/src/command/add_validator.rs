use std::str::FromStr;

use clap::Args;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey, stake, system_program, sysvar};
use solana_sdk::instruction::Instruction;
use spl_stake_pool_legacy::{find_stake_program_address, state::StakePool};

use crate::{
    checked_transaction_with_signers,
    client::{get_stake_pool, get_validator_list},
    config::JitoStakePoolCliConfig,
};

#[derive(Args)]
pub struct AddValidatorArgs {
    /// Stake pool address
    pub pool: String,

    /// The validator vote account that the stake is delegated to
    pub vote_account: String,
}

pub fn add_validator_to_pool_with_vote(
    program_id: &Pubkey,
    stake_pool: &StakePool,
    stake_pool_address: &Pubkey,
    funder: &Pubkey,
    vote_account_address: &Pubkey,
) -> Instruction {
    let pool_withdraw_authority =
        find_withdraw_authority_program_address(program_id, stake_pool_address).0;
    let (stake_account_address, _) =
        find_stake_program_address(program_id, vote_account_address, stake_pool_address);
    add_validator_to_pool(
        program_id,
        stake_pool_address,
        &stake_pool.staker,
        funder,
        &pool_withdraw_authority,
        &stake_pool.validator_list,
        &stake_account_address,
        vote_account_address,
    )
}

pub fn add_validator_to_pool(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    staker: &Pubkey,
    funder: &Pubkey,
    stake_pool_withdraw: &Pubkey,
    validator_list: &Pubkey,
    stake: &Pubkey,
    validator: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*stake_pool, false),
        AccountMeta::new_readonly(*staker, true),
        AccountMeta::new(*funder, true),
        AccountMeta::new_readonly(*stake_pool_withdraw, false),
        AccountMeta::new(*validator_list, false),
        AccountMeta::new(*stake, false),
        AccountMeta::new_readonly(*validator, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::stake_history::id(), false),
        AccountMeta::new_readonly(stake::config::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(stake::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: StakePoolInstruction::AddValidatorToPool
            .try_to_vec()
            .unwrap(),
    }
}

/// Seed for deposit authority seed
const AUTHORITY_DEPOSIT: &[u8] = b"deposit";

/// Seed for withdraw authority seed
const AUTHORITY_WITHDRAW: &[u8] = b"withdraw";

/// Seed for transient stake account
const TRANSIENT_STAKE_SEED_PREFIX: &[u8] = b"transient";

// Minimum amount of staked SOL required in a validator stake account to allow
// for merges without a mismatch on credits observed
// pub const MINIMUM_ACTIVE_STAKE: u64 = LAMPORTS_PER_SOL / 1_000;

/// Maximum amount of validator stake accounts to update per
/// `UpdateValidatorListBalance` instruction, based on compute limits
pub const MAX_VALIDATORS_TO_UPDATE: usize = 5;

// Maximum factor by which a withdrawal fee can be increased per epoch
// protecting stakers from malicious users.
// If current fee is 0, WITHDRAWAL_BASELINE_FEE is used as the baseline
// pub const MAX_WITHDRAWAL_FEE_INCREASE: Fee = Fee {
//     numerator: 3,
//     denominator: 2,
// };
// /// Drop-in baseline fee when evaluating withdrawal fee increases when fee is 0
// pub const WITHDRAWAL_BASELINE_FEE: Fee = Fee {
//     numerator: 1,
//     denominator: 1000,
// };

/// The maximum number of transient stake accounts respecting
/// transaction account limits.
pub const MAX_TRANSIENT_STAKE_ACCOUNTS: usize = 10;

// /// Get the stake amount under consideration when calculating pool token
// /// conversions
// #[inline]
// pub fn minimum_stake_lamports(meta: &Meta) -> u64 {
//     meta.rent_exempt_reserve
//         .saturating_add(MINIMUM_ACTIVE_STAKE)
// }

// /// Get the stake amount under consideration when calculating pool token
// /// conversions
// #[inline]
// pub fn minimum_reserve_lamports(meta: &Meta) -> u64 {
//     meta.rent_exempt_reserve.saturating_add(1)
// }

/// Generates the deposit authority program address for the stake pool
pub fn find_deposit_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&stake_pool_address.to_bytes()[..32], AUTHORITY_DEPOSIT],
        program_id,
    )
}

/// Generates the withdraw authority program address for the stake pool
pub fn find_withdraw_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&stake_pool_address.to_bytes(), AUTHORITY_WITHDRAW],
        program_id,
    )
}

// /// Generates the stake program address for a validator's vote account
// pub fn find_stake_program_address(
//     program_id: &Pubkey,
//     vote_account_address: &Pubkey,
//     stake_pool_address: &Pubkey,
// ) -> (Pubkey, u8) {
//     Pubkey::find_program_address(
//         &[
//             &vote_account_address.to_bytes(),
//             &stake_pool_address.to_bytes(),
//         ],
//         program_id,
//     )
// }

/// Generates the stake program address for a validator's vote account
pub fn find_transient_stake_program_address(
    program_id: &Pubkey,
    vote_account_address: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            TRANSIENT_STAKE_SEED_PREFIX,
            &vote_account_address.to_bytes(),
            &stake_pool_address.to_bytes(),
            &seed.to_le_bytes(),
        ],
        program_id,
    )
}

solana_program::declare_id!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy");

pub fn command_vsa_add(
    config: &JitoStakePoolCliConfig,
    stake_pool_address: &Pubkey,
    vote_account: &Pubkey,
) -> anyhow::Result<()> {
    let vote_account = Pubkey::new_from_array(vote_account.to_bytes());
    // let (stake_account_address, _) =
    //     find_stake_program_address(&spl_stake_pool::id(), vote_account, stake_pool_address);

    let program_id = Pubkey::from_str("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy").unwrap();

    let stake_account_address = Pubkey::find_program_address(
        &[&vote_account.to_bytes(), &stake_pool_address.to_bytes()],
        &program_id,
    )
    .0;

    println!(
        "Adding stake account {}, delegated to {}",
        stake_account_address, vote_account
    );

    let stake_pool = get_stake_pool(&config.rpc_client, stake_pool_address)?;
    let validator_list = Pubkey::new_from_array(stake_pool.validator_list.to_bytes());

    let validator_list = get_validator_list(&config.rpc_client, &validator_list)?;

    if validator_list.contains(&vote_account) {
        println!(
            "Stake pool already contains validator {}, ignoring",
            vote_account
        );
        return Ok(());
    }

    //     if !config.no_update {
    //         command_update(config, stake_pool_address, false, false)?;
    //     }
    //
    let mut signers = vec![config.fee_payer.as_ref(), config.staker.as_ref()];

    let fee_payer = Pubkey::new_from_array(config.fee_payer.pubkey().to_bytes());

    // unique_signers!(signers);
    let transaction = checked_transaction_with_signers(
        config,
        &[add_validator_to_pool_with_vote(
            &spl_stake_pool_legacy::id(),
            &stake_pool,
            stake_pool_address,
            &fee_payer,
            &vote_account,
        )],
        &signers,
    )?;

    send_transaction(config, transaction)?;

    Ok(())
}
