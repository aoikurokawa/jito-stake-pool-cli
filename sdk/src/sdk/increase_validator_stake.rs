use borsh_legacy::BorshSerialize;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    stake, system_program, sysvar,
};
use spl_stake_pool::{
    find_stake_program_address, find_transient_stake_program_address,
    find_withdraw_authority_program_address, state::StakePool,
};

pub fn increase_validator_stake_with_vote(
    program_id: &Pubkey,
    stake_pool: &StakePool,
    stake_pool_address: &Pubkey,
    vote_account_address: &Pubkey,
    lamports: u64,
    transient_stake_seed: u64,
) -> Instruction {
    let pool_withdraw_authority =
        find_withdraw_authority_program_address(program_id, stake_pool_address).0;
    let (transient_stake_address, _) = find_transient_stake_program_address(
        program_id,
        vote_account_address,
        stake_pool_address,
        transient_stake_seed,
    );
    let (validator_stake_address, _) =
        find_stake_program_address(program_id, vote_account_address, stake_pool_address, None);

    increase_validator_stake(
        program_id,
        stake_pool_address,
        &stake_pool.staker,
        &pool_withdraw_authority,
        &stake_pool.validator_list,
        &stake_pool.reserve_stake,
        &transient_stake_address,
        &validator_stake_address,
        lamports,
        transient_stake_seed,
    )
}

pub fn increase_validator_stake(
    program_id: &Pubkey,
    stake_pool: &Pubkey,
    staker: &Pubkey,
    stake_pool_withdraw_authority: &Pubkey,
    validator_list: &Pubkey,
    reserve_stake: &Pubkey,
    transient_stake: &Pubkey,
    validator: &Pubkey,
    lamports: u64,
    transient_stake_seed: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*stake_pool, false),
        AccountMeta::new_readonly(*staker, true),
        AccountMeta::new_readonly(*stake_pool_withdraw_authority, false),
        AccountMeta::new(*validator_list, false),
        AccountMeta::new(*reserve_stake, false),
        AccountMeta::new(*transient_stake, false),
        AccountMeta::new_readonly(*validator, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::stake_history::id(), false),
        AccountMeta::new_readonly(stake::config::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(stake::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: spl_stake_pool_legacy::instruction::StakePoolInstruction::IncreaseValidatorStake {
            lamports,
            transient_stake_seed,
        }
        .try_to_vec()
        .unwrap(),
    }
}
