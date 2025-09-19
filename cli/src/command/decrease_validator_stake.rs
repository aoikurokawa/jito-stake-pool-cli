use clap::Args;

#[derive(Args)]
struct DecreaseValidatorStakeArgs {
    /// Stake pool address
    pool: String,
    /// Vote account for the validator to decrease stake from
    vote_account: String,
    /// Amount in SOL to remove from the validator stake account. Must be at least the rent-exempt amount for a stake.
    amount: Option<f64>,
}
