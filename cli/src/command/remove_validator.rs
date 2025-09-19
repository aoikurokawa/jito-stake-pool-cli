use clap::Args;

#[derive(Args)]
struct RemoveValidatorArgs {
    /// Stake pool address
    pool: String,
    /// Vote account for the validator to remove from the pool
    vote_account: String,
    /// New authority to set as Staker and Withdrawer in the stake account removed from the pool. Defaults to the client keypair.
    #[arg(long = "new-authority", value_name = "ADDRESS")]
    new_authority: Option<String>,
    /// Stake account to receive SOL from the stake pool. Defaults to a new stake account.
    #[arg(long = "stake-receiver", value_name = "ADDRESS")]
    stake_receiver: Option<String>,
}
