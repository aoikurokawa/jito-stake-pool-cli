use clap::Args;

#[derive(Args)]
pub struct CreatePoolArgs {
    /// Epoch fee numerator, fee amount is numerator divided by denominator.
    #[arg(long = "epoch-fee-numerator", short = 'n', value_name = "NUMERATOR")]
    epoch_fee_numerator: u64,

    /// Epoch fee denominator, fee amount is numerator divided by denominator.
    #[arg(
        long = "epoch-fee-denominator",
        short = 'd',
        value_name = "DENOMINATOR"
    )]
    epoch_fee_denominator: u64,

    /// Withdrawal fee numerator, fee amount is numerator divided by denominator [default: 0]
    #[arg(
        long = "withdrawal-fee-numerator",
        value_name = "NUMERATOR",
        requires = "withdrawal_fee_denominator"
    )]
    withdrawal_fee_numerator: Option<u64>,

    /// Withdrawal fee denominator, fee amount is numerator divided by denominator [default: 0]
    #[arg(
        long = "withdrawal-fee-denominator",
        value_name = "DENOMINATOR",
        requires = "withdrawal_fee_numerator"
    )]
    withdrawal_fee_denominator: Option<u64>,

    /// Deposit fee numerator, fee amount is numerator divided by denominator [default: 0]
    #[arg(
        long = "deposit-fee-numerator",
        value_name = "NUMERATOR",
        requires = "deposit_fee_denominator"
    )]
    deposit_fee_numerator: Option<u64>,

    /// Deposit fee denominator, fee amount is numerator divided by denominator [default: 0]
    #[arg(
        long = "deposit-fee-denominator",
        value_name = "DENOMINATOR",
        requires = "deposit_fee_numerator"
    )]
    deposit_fee_denominator: Option<u64>,

    /// Referral fee percentage, maximum 100
    #[arg(long = "referral-fee", value_name = "FEE_PERCENTAGE")]
    referral_fee: Option<u8>,

    /// Max number of validators included in the stake pool
    #[arg(long = "max-validators", short = 'm', value_name = "NUMBER")]
    max_validators: u32,

    /// Deposit authority required to sign all deposits into the stake pool
    #[arg(
        long = "deposit-authority",
        short = 'a',
        value_name = "DEPOSIT_AUTHORITY_KEYPAIR"
    )]
    deposit_authority: Option<String>,

    /// Stake pool keypair [default: new keypair]
    #[arg(long = "pool-keypair", short = 'p', value_name = "PATH")]
    pool_keypair: Option<String>,

    /// Validator list keypair [default: new keypair]
    #[arg(long = "validator-list-keypair", value_name = "PATH")]
    validator_list_keypair: Option<String>,

    /// Stake pool mint keypair [default: new keypair]
    #[arg(long = "mint-keypair", value_name = "PATH")]
    mint_keypair: Option<String>,

    /// Stake pool reserve keypair [default: new keypair]
    #[arg(long = "reserve-keypair", value_name = "PATH")]
    reserve_keypair: Option<String>,

    /// Bypass fee checks, allowing pool to be created with unsafe fees
    #[arg(long = "unsafe-fees")]
    unsafe_fees: bool,
}

#[allow(clippy::too_many_arguments)]
fn command_create_pool(
    config: &Config,
    deposit_authority: Option<Keypair>,
    epoch_fee: Fee,
    withdrawal_fee: Fee,
    deposit_fee: Fee,
    referral_fee: u8,
    max_validators: u32,
    stake_pool_keypair: Option<Keypair>,
    validator_list_keypair: Option<Keypair>,
    mint_keypair: Option<Keypair>,
    reserve_keypair: Option<Keypair>,
    unsafe_fees: bool,
) -> CommandResult {
    if !unsafe_fees {
        check_stake_pool_fees(&epoch_fee, &withdrawal_fee, &deposit_fee)?;
    }
    let reserve_keypair = reserve_keypair.unwrap_or_else(Keypair::new);
    println!("Creating reserve stake {}", reserve_keypair.pubkey());

    let mint_keypair = mint_keypair.unwrap_or_else(Keypair::new);
    println!("Creating mint {}", mint_keypair.pubkey());

    let stake_pool_keypair = stake_pool_keypair.unwrap_or_else(Keypair::new);

    let validator_list_keypair = validator_list_keypair.unwrap_or_else(Keypair::new);

    let reserve_stake_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(STAKE_STATE_LEN)?
        + 1;
    let mint_account_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;
    let pool_fee_account_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?;
    let stake_pool_account_lamports = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(get_packed_len::<StakePool>())?;
    let empty_validator_list = ValidatorList::new(max_validators);
    let validator_list_size = get_instance_packed_len(&empty_validator_list)?;
    let validator_list_balance = config
        .rpc_client
        .get_minimum_balance_for_rent_exemption(validator_list_size)?;
    let mut total_rent_free_balances = reserve_stake_balance
        + mint_account_balance
        + pool_fee_account_balance
        + stake_pool_account_lamports
        + validator_list_balance;

    let default_decimals = spl_token::native_mint::DECIMALS;

    // Calculate withdraw authority used for minting pool tokens
    let (withdraw_authority, _) = find_withdraw_authority_program_address(
        &spl_stake_pool::id(),
        &stake_pool_keypair.pubkey(),
    );

    if config.verbose {
        println!("Stake pool withdraw authority {}", withdraw_authority);
    }

    let mut instructions = vec![
        // Account for the stake pool reserve
        system_instruction::create_account(
            &config.fee_payer.pubkey(),
            &reserve_keypair.pubkey(),
            reserve_stake_balance,
            STAKE_STATE_LEN as u64,
            &stake::program::id(),
        ),
        stake::instruction::initialize(
            &reserve_keypair.pubkey(),
            &stake::state::Authorized {
                staker: withdraw_authority,
                withdrawer: withdraw_authority,
            },
            &stake::state::Lockup::default(),
        ),
        // Account for the stake pool mint
        system_instruction::create_account(
            &config.fee_payer.pubkey(),
            &mint_keypair.pubkey(),
            mint_account_balance,
            spl_token::state::Mint::LEN as u64,
            &spl_token::id(),
        ),
        // Initialize pool token mint account
        spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &mint_keypair.pubkey(),
            &withdraw_authority,
            None,
            default_decimals,
        )?,
    ];

    let pool_fee_account = add_associated_token_account(
        config,
        &mint_keypair.pubkey(),
        &config.manager.pubkey(),
        &mut instructions,
        &mut total_rent_free_balances,
    );
    println!("Creating pool fee collection account {}", pool_fee_account);

    let recent_blockhash = get_latest_blockhash(&config.rpc_client)?;
    let setup_message = Message::new_with_blockhash(
        &instructions,
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );
    let initialize_message = Message::new_with_blockhash(
        &[
            // Validator stake account list storage
            system_instruction::create_account(
                &config.fee_payer.pubkey(),
                &validator_list_keypair.pubkey(),
                validator_list_balance,
                validator_list_size as u64,
                &spl_stake_pool::id(),
            ),
            // Account for the stake pool
            system_instruction::create_account(
                &config.fee_payer.pubkey(),
                &stake_pool_keypair.pubkey(),
                stake_pool_account_lamports,
                get_packed_len::<StakePool>() as u64,
                &spl_stake_pool::id(),
            ),
            // Initialize stake pool
            spl_stake_pool::instruction::initialize(
                &spl_stake_pool::id(),
                &stake_pool_keypair.pubkey(),
                &config.manager.pubkey(),
                &config.staker.pubkey(),
                &withdraw_authority,
                &validator_list_keypair.pubkey(),
                &reserve_keypair.pubkey(),
                &mint_keypair.pubkey(),
                &pool_fee_account,
                &spl_token::id(),
                deposit_authority.as_ref().map(|x| x.pubkey()),
                epoch_fee,
                withdrawal_fee,
                deposit_fee,
                referral_fee,
                max_validators,
            ),
        ],
        Some(&config.fee_payer.pubkey()),
        &recent_blockhash,
    );
    check_fee_payer_balance(
        config,
        total_rent_free_balances
            + config.rpc_client.get_fee_for_message(&setup_message)?
            + config.rpc_client.get_fee_for_message(&initialize_message)?,
    )?;
    let mut setup_signers = vec![config.fee_payer.as_ref(), &mint_keypair, &reserve_keypair];
    unique_signers!(setup_signers);
    let setup_transaction = Transaction::new(&setup_signers, setup_message, recent_blockhash);
    let mut initialize_signers = vec![
        config.fee_payer.as_ref(),
        &stake_pool_keypair,
        &validator_list_keypair,
        config.manager.as_ref(),
    ];
    let initialize_transaction = if let Some(deposit_authority) = deposit_authority {
        println!(
            "Deposits will be restricted to {} only, this can be changed using the set-funding-authority command.",
            deposit_authority.pubkey()
        );
        let mut initialize_signers = initialize_signers.clone();
        initialize_signers.push(&deposit_authority);
        unique_signers!(initialize_signers);
        Transaction::new(&initialize_signers, initialize_message, recent_blockhash)
    } else {
        unique_signers!(initialize_signers);
        Transaction::new(&initialize_signers, initialize_message, recent_blockhash)
    };
    send_transaction(config, setup_transaction)?;

    println!(
        "Creating stake pool {} with validator list {}",
        stake_pool_keypair.pubkey(),
        validator_list_keypair.pubkey()
    );
    send_transaction(config, initialize_transaction)?;
    Ok(())
}
