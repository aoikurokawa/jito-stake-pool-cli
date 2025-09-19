pub mod add_validator;
// pub mod create_pool;
pub mod increase_validator_stake;
pub mod remove_validator;

// #[derive(Args)]
// struct SetPreferredValidatorArgs {
//     /// Stake pool address
//     pool: String,
//     /// Operation for which to restrict the validator
//     #[arg(value_enum)]
//     preferred_type: PreferredType,
//     /// Vote account for the validator that users must deposit into.
//     #[arg(
//         long = "vote-account",
//         value_name = "VOTE_ACCOUNT_ADDRESS",
//         group = "validator"
//     )]
//     vote_account: Option<String>,
//     /// Unset the preferred validator.
//     #[arg(long, group = "validator")]
//     unset: bool,
// }
//
// #[derive(clap::ValueEnum, Clone)]
// enum PreferredType {
//     Deposit,
//     Withdraw,
// }
//
// #[derive(Args)]
// struct DepositStakeArgs {
//     /// Stake pool address
//     pool: String,
//     /// Stake address to join the pool
//     stake_account: String,
//     /// Withdraw authority for the stake account to be deposited. [default: cli config keypair]
//     #[arg(long = "withdraw-authority", value_name = "KEYPAIR")]
//     withdraw_authority: Option<String>,
//     /// Account to receive the minted pool tokens. Defaults to the token-owner's associated pool token account. Creates the account if it does not exist.
//     #[arg(long = "token-receiver", value_name = "ADDRESS")]
//     token_receiver: Option<String>,
//     /// Pool token account to receive the referral fees for deposits. Defaults to the token receiver.
//     #[arg(value_name = "ADDRESS")]
//     referrer: Option<String>,
// }
//
// #[derive(Args)]
// struct DepositAllStakeArgs {
//     /// Stake pool address
//     pool: String,
//     /// Stake authority address to search for stake accounts
//     stake_authority: String,
//     /// Withdraw authority for the stake account to be deposited. [default: cli config keypair]
//     #[arg(long = "withdraw-authority", value_name = "KEYPAIR")]
//     withdraw_authority: Option<String>,
//     /// Account to receive the minted pool tokens. Defaults to the token-owner's associated pool token account. Creates the account if it does not exist.
//     #[arg(long = "token-receiver", value_name = "ADDRESS")]
//     token_receiver: Option<String>,
//     /// Pool token account to receive the referral fees for deposits. Defaults to the token receiver.
//     #[arg(value_name = "ADDRESS")]
//     referrer: Option<String>,
// }
//
// #[derive(Args)]
// struct DepositSolArgs {
//     /// Stake pool address
//     pool: String,
//     /// Amount in SOL to deposit into the stake pool reserve account.
//     amount: Option<f64>,
//     /// Source account of funds. [default: cli config keypair]
//     #[arg(long, value_name = "KEYPAIR")]
//     from: Option<String>,
//     /// Account to receive the minted pool tokens. Defaults to the token-owner's associated pool token account. Creates the account if it does not exist.
//     #[arg(long = "token-receiver", value_name = "POOL_TOKEN_RECEIVER_ADDRESS")]
//     token_receiver: Option<String>,
//     /// Account to receive the referral fees for deposits. Defaults to the token receiver.
//     #[arg(long, value_name = "REFERRER_TOKEN_ADDRESS")]
//     referrer: Option<String>,
// }
//
// #[derive(Args)]
// struct ListArgs {
//     /// Stake pool address.
//     pool: String,
// }
//
// #[derive(Args)]
// struct UpdateArgs {
//     /// Stake pool address.
//     pool: String,
//     /// Update all balances, even if it has already been performed this epoch.
//     #[arg(long)]
//     force: bool,
//     /// Do not automatically merge transient stakes. Useful if the stake pool is in an expected state, but the balances still need to be updated.
//     #[arg(long = "no-merge")]
//     no_merge: bool,
// }
//
// #[derive(Args)]
// struct WithdrawStakeArgs {
//     /// Stake pool address.
//     pool: String,
//     /// Amount of pool tokens to withdraw for activated stake.
//     amount: f64,
//     /// Pool token account to withdraw tokens from. Defaults to the token-owner's associated token account.
//     #[arg(long = "pool-account", value_name = "ADDRESS")]
//     pool_account: Option<String>,
//     /// Stake account from which to receive a stake from the stake pool. Defaults to a new stake account.
//     #[arg(
//         long = "stake-receiver",
//         value_name = "STAKE_ACCOUNT_ADDRESS",
//         requires = "withdraw_from"
//     )]
//     stake_receiver: Option<String>,
//     /// Validator to withdraw from. Defaults to the largest validator stakes in the pool.
//     #[arg(
//         long = "vote-account",
//         value_name = "VOTE_ACCOUNT_ADDRESS",
//         group = "withdraw_from"
//     )]
//     vote_account: Option<String>,
//     /// Withdraw from the stake pool's reserve. Only possible if all validator stakes are at the minimum possible amount.
//     #[arg(long = "use-reserve", group = "withdraw_from")]
//     use_reserve: bool,
// }
//
// #[derive(Args)]
// struct WithdrawSolArgs {
//     /// Stake pool address.
//     pool: String,
//     /// System account to receive SOL from the stake pool. Defaults to the payer.
//     sol_receiver: String,
//     /// Amount of pool tokens to withdraw for SOL.
//     amount: f64,
//     /// Pool token account to withdraw tokens from. Defaults to the token-owner's associated token account.
//     #[arg(long = "pool-account", value_name = "ADDRESS")]
//     pool_account: Option<String>,
// }
//
// #[derive(Args)]
// struct SetManagerArgs {
//     /// Stake pool address.
//     pool: String,
//     /// Keypair for the new stake pool manager.
//     #[arg(long = "new-manager", value_name = "KEYPAIR", group = "new_accounts")]
//     new_manager: Option<String>,
//     /// Public key for the new account to set as the stake pool fee receiver.
//     #[arg(
//         long = "new-fee-receiver",
//         value_name = "ADDRESS",
//         group = "new_accounts"
//     )]
//     new_fee_receiver: Option<String>,
// }
//
// #[derive(Args)]
// struct SetStakerArgs {
//     /// Stake pool address.
//     pool: String,
//     /// Public key for the new stake pool staker.
//     new_staker: String,
// }
//
// #[derive(Args)]
// struct SetFundingAuthorityArgs {
//     /// Stake pool address.
//     pool: String,
//     /// Funding type to be updated.
//     #[arg(value_enum)]
//     funding_type: FundingTypeArg,
//     /// Public key for the new stake pool funding authority.
//     #[arg(group = "validator")]
//     new_authority: Option<String>,
//     /// Unset the stake deposit authority. The program will use a program derived address.
//     #[arg(long, group = "validator")]
//     unset: bool,
// }
//
// #[derive(clap::ValueEnum, Clone)]
// enum FundingTypeArg {
//     StakeDeposit,
//     SolDeposit,
//     SolWithdraw,
// }
//
// #[derive(Args)]
// struct SetFeeArgs {
//     /// Stake pool address.
//     pool: String,
//     /// Fee type to be updated.
//     #[arg(value_enum)]
//     fee_type: FeeTypeArg,
//     /// Fee numerator, fee amount is numerator divided by denominator.
//     fee_numerator: u64,
//     /// Fee denominator, fee amount is numerator divided by denominator.
//     fee_denominator: u64,
// }
//
// #[derive(clap::ValueEnum, Clone)]
// enum FeeTypeArg {
//     Epoch,
//     StakeDeposit,
//     SolDeposit,
//     StakeWithdrawal,
//     SolWithdrawal,
// }
//
// #[derive(Args)]
// struct SetReferralFeeArgs {
//     /// Stake pool address.
//     pool: String,
//     /// Fee type to be updated.
//     #[arg(value_enum)]
//     fee_type: ReferralFeeTypeArg,
//     /// Fee percentage, maximum 100
//     fee: u8,
// }
//
// #[derive(clap::ValueEnum, Clone)]
// enum ReferralFeeTypeArg {
//     Stake,
//     Sol,
// }
