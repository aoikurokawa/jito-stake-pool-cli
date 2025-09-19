use borsh_legacy::BorshSerialize;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};
use spl_stake_pool::{
    find_stake_program_address, find_withdraw_authority_program_address, state::StakePool,
};

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
        find_stake_program_address(program_id, vote_account_address, stake_pool_address, None);
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

#[allow(clippy::too_many_arguments)]
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
        #[allow(deprecated)]
        AccountMeta::new_readonly(solana_stake_interface::config::id(), false),
        AccountMeta::new_readonly(solana_system_interface::program::id(), false),
        AccountMeta::new_readonly(solana_stake_interface::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: spl_stake_pool_legacy::instruction::StakePoolInstruction::AddValidatorToPool
            .try_to_vec()
            .unwrap(),
    }
}
