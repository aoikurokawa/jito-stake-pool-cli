use anyhow::anyhow;
use borsh::BorshDeserialize;
// use solana_client::{
//     client_error::ClientError,
//     rpc_client::RpcClient,
//     rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
// };
// use solana_program::{borsh::try_from_slice_unchecked, program_pack::Pack, stake};
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_stake_pool::state::{StakePool, ValidatorList};
// use spl_stake_pool::{
//     find_withdraw_authority_program_address,
//     state::{StakePool, ValidatorList},
// };
// use spl_stake_pool_legacy::state::ValidatorList;

pub fn get_stake_pool(
    rpc_client: &RpcClient,
    stake_pool_address: &Pubkey,
) -> anyhow::Result<StakePool> {
    let account_data = rpc_client.get_account_data(stake_pool_address)?;
    let stake_pool = StakePool::deserialize(&mut account_data.as_slice())
        .map_err(|err| anyhow!("Invalid stake pool {}: {}", stake_pool_address, err))?;
    Ok(stake_pool)
}

pub fn get_validator_list(
    rpc_client: &RpcClient,
    validator_list_address: &Pubkey,
) -> anyhow::Result<ValidatorList> {
    let account_data = rpc_client.get_account_data(validator_list_address)?;
    let validator_list = ValidatorList::deserialize(&mut account_data.as_slice())
        .map_err(|err| anyhow!("Invalid validator list {}: {}", validator_list_address, err))?;
    Ok(validator_list)
}

// pub fn get_token_account(
//     rpc_client: &RpcClient,
//     token_account_address: &Pubkey,
//     expected_token_mint: &Pubkey,
// ) -> Result<spl_token::state::Account, Error> {
//     let account_data = rpc_client.get_account_data(token_account_address)?;
//     let token_account = spl_token::state::Account::unpack_from_slice(account_data.as_slice())
//         .map_err(|err| format!("Invalid token account {}: {}", token_account_address, err))?;
//
//     if token_account.mint != *expected_token_mint {
//         Err(format!(
//             "Invalid token mint for {}, expected mint is {}",
//             token_account_address, expected_token_mint
//         )
//         .into())
//     } else {
//         Ok(token_account)
//     }
// }
//
// pub fn get_token_mint(
//     rpc_client: &RpcClient,
//     token_mint_address: &Pubkey,
// ) -> Result<spl_token::state::Mint, Error> {
//     let account_data = rpc_client.get_account_data(token_mint_address)?;
//     let token_mint = spl_token::state::Mint::unpack_from_slice(account_data.as_slice())
//         .map_err(|err| format!("Invalid token mint {}: {}", token_mint_address, err))?;
//
//     Ok(token_mint)
// }
//
// pub(crate) fn get_stake_state(
//     rpc_client: &RpcClient,
//     stake_address: &Pubkey,
// ) -> Result<stake::state::StakeState, Error> {
//     let account_data = rpc_client.get_account_data(stake_address)?;
//     let stake_state = deserialize(account_data.as_slice())
//         .map_err(|err| format!("Invalid stake account {}: {}", stake_address, err))?;
//     Ok(stake_state)
// }

// pub(crate) fn get_stake_pools(
//     rpc_client: &RpcClient,
// ) -> Result<Vec<(Pubkey, StakePool, ValidatorList, Pubkey)>, ClientError> {
//     rpc_client
//         .get_program_accounts_with_config(
//             &spl_stake_pool::id(),
//             RpcProgramAccountsConfig {
//                 filters: Some(vec![RpcFilterType::Memcmp(Memcmp {
//                     offset: 0, // 0 is the account type
//                     bytes: MemcmpEncodedBytes::Base58("2".to_string()),
//                     encoding: None,
//                 })]),
//                 account_config: RpcAccountInfoConfig {
//                     encoding: Some(UiAccountEncoding::Base64),
//                     ..RpcAccountInfoConfig::default()
//                 },
//                 ..RpcProgramAccountsConfig::default()
//             },
//         )
//         .map(|accounts| {
//             accounts
//                 .into_iter()
//                 .filter_map(|(address, account)| {
//                     let pool_withdraw_authority =
//                         find_withdraw_authority_program_address(&spl_stake_pool::id(), &address).0;
//                     match try_from_slice_unchecked::<StakePool>(account.data.as_slice()) {
//                         Ok(stake_pool) => {
//                             get_validator_list(rpc_client, &stake_pool.validator_list)
//                                 .map(|validator_list| {
//                                     (address, stake_pool, validator_list, pool_withdraw_authority)
//                                 })
//                                 .ok()
//                         }
//                         Err(err) => {
//                             eprintln!("Invalid stake pool data for {}: {}", address, err);
//                             None
//                         }
//                     }
//                 })
//                 .collect()
//         })
// }

// pub(crate) fn get_all_stake(
//     rpc_client: &RpcClient,
//     authorized_staker: &Pubkey,
// ) -> Result<HashSet<Pubkey>, ClientError> {
//     let all_stake_accounts = rpc_client.get_program_accounts_with_config(
//         &stake::program::id(),
//         RpcProgramAccountsConfig {
//             filters: Some(vec![
//                 // Filter by `Meta::authorized::staker`, which begins at byte offset 12
//                 RpcFilterType::Memcmp(Memcmp {
//                     offset: 12,
//                     bytes: MemcmpEncodedBytes::Base58(authorized_staker.to_string()),
//                     encoding: Some(MemcmpEncoding::Binary),
//                 }),
//             ]),
//             account_config: RpcAccountInfoConfig {
//                 encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
//                 commitment: Some(rpc_client.commitment()),
//                 ..RpcAccountInfoConfig::default()
//             },
//             ..RpcProgramAccountsConfig::default()
//         },
//     )?;
//
//     Ok(all_stake_accounts
//         .into_iter()
//         .map(|(address, _)| address)
//         .collect())
// }
