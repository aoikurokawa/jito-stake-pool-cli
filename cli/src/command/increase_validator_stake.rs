use clap::Args;

#[derive(Args)]
pub struct IncreaseValidatorStakeArgs {
    /// Stake pool address
    pub pool: String,

    /// Vote account for the validator to increase stake to
    pub vote_account: String,

    /// Amount in SOL to add to the validator stake account. Must be at least the rent-exempt amount for a stake plus 1 SOL for merging.
    pub amount: Option<f64>,
}
