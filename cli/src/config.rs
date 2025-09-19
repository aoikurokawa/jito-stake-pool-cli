use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::signer::Signer;

pub struct JitoStakePoolCliConfig {
    /// RPC Client
    pub rpc_client: RpcClient,

    /// Verbose
    pub verbose: bool,

    // output_format: OutputFormat,
    /// Manager
    pub manager: Box<dyn Signer>,

    /// Staker
    pub staker: Box<dyn Signer>,

    /// Funding authority
    pub funding_authority: Option<Box<dyn Signer>>,

    /// Token owner
    pub token_owner: Box<dyn Signer>,

    /// Fee payer
    pub fee_payer: Box<dyn Signer>,

    /// Dry run
    pub dry_run: bool,

    /// No update
    pub no_update: bool,
}
