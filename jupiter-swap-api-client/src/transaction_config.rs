use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use solana_account_decoder::UiAccount;
use solana_sdk::pubkey::Pubkey;

use crate::serde_helpers::option_field_as_string;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ComputeUnitPriceMicroLamports {
    MicroLamports(u64),
    #[serde(deserialize_with = "auto")]
    Auto,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
// #[serde(untagged)]
pub enum PrioritizationFeeLamports {
    /// Jupiter will automatically set a priority fee,
    /// and it will be capped at 5,000,000 lamports / 0.005 SOL
    #[serde(deserialize_with = "auto")]
    Auto,
    /// The priority fee will be a multiplier on the auto fee.
    AutoMultiplier(u64),
    /// A tip instruction will be included to Jito and no priority fee will be set.
    JitoTipLamports(u64),
}

fn auto<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    enum Helper {
        #[serde(rename = "auto")]
        Variant,
    }
    Helper::deserialize(deserializer)?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSlippageSettings {
    pub min_bps: Option<u16>,
    pub max_bps: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TransactionConfig {
    /// Wrap and unwrap SOL. Will be ignored if `destination_token_account` is set because the `destination_token_account` may belong to a different user that we have no authority to close.
    pub wrap_and_unwrap_sol: bool,
    /// Allow optimized WSOL token account by using transfer, assign with seed, allocate with seed then initialize account 3 instead of the expensive associated token account process
    pub allow_optimized_wrapped_sol_token_account: bool,
    /// Fee token account for the output token, it is derived using the seeds = ["referral_ata", referral_account, mint] and the `REFER4ZgmyYx9c6He5XfaTMiGfdLwRnkV4RPp9t9iF3` referral contract (only pass in if you set a feeBps and make sure that the feeAccount has been created)
    #[serde(with = "option_field_as_string")]
    pub fee_account: Option<Pubkey>,
    /// Public key of the token account that will be used to receive the token out of the swap. If not provided, the user's ATA will be used. If provided, we assume that the token account is already initialized.
    #[serde(with = "option_field_as_string")]
    pub destination_token_account: Option<Pubkey>,
    /// Add a readonly, non signer tracking account that isn't used by jupiter
    #[serde(with = "option_field_as_string")]
    pub tracking_account: Option<Pubkey>,
    /// compute unit price to prioritize the transaction, the additional fee will be compute unit consumed * computeUnitPriceMicroLamports
    pub compute_unit_price_micro_lamports: Option<ComputeUnitPriceMicroLamports>,
    /// Prioritization fee lamports paid for the transaction in addition to the signatures fee.
    /// Mutually exclusive with `compute_unit_price_micro_lamports`.
    pub prioritization_fee_lamports: Option<PrioritizationFeeLamports>,
    /// When enabled, it will do a swap simulation to get the compute unit used and set it in ComputeBudget's compute unit limit.
    /// This will increase latency slightly since there will be one extra RPC call to simulate this. Default is false.
    pub dynamic_compute_unit_limit: bool,
    /// Request a legacy transaction rather than the default versioned transaction, needs to be paired with a quote using asLegacyTransaction otherwise the transaction might be too large
    ///
    /// Default: false
    pub as_legacy_transaction: bool,
    /// This enables the usage of shared program accounts. That means no intermediate token accounts or open orders accounts need to be created.
    /// But it also means that the likelihood of hot accounts is higher.
    ///
    /// Default: true
    pub use_shared_accounts: bool,
    /// This is useful when the instruction before the swap has a transfer that increases the input token amount.
    /// Then, the swap will just use the difference between the token ledger token amount and post token amount.
    ///
    /// Default: false
    pub use_token_ledger: bool,
    /// Skip RPC calls and assume the user account do not exist,
    /// as a result all setup instruction will be populated but no RPC call will be done for user related accounts (token accounts, openbook open orders...)
    pub skip_user_accounts_rpc_calls: bool,
    /// Providing keyed ui accounts allow loading AMMs that are not in the market cache
    /// If a keyed ui account is the AMM state, it has to be provided with its params according to the market cache format
    pub keyed_ui_accounts: Option<Vec<KeyedUiAccount>>,
    /// The program authority ID
    pub program_authority_id: Option<u8>,
    /// Dynamic slippage
    pub dynamic_slippage: Option<DynamicSlippageSettings>,
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            wrap_and_unwrap_sol: true,
            fee_account: None,
            destination_token_account: None,
            compute_unit_price_micro_lamports: None,
            prioritization_fee_lamports: None,
            dynamic_compute_unit_limit: false,
            as_legacy_transaction: false,
            use_shared_accounts: true,
            use_token_ledger: false,
            allow_optimized_wrapped_sol_token_account: false,
            tracking_account: None,
            skip_user_accounts_rpc_calls: false,
            keyed_ui_accounts: None,
            program_authority_id: None,
            dynamic_slippage: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyedUiAccount {
    pub pubkey: String,
    #[serde(flatten)]
    pub ui_account: UiAccount,
    /// Additional data an Amm requires, Amm dependent and decoded in the Amm implementation
    pub params: Option<Value>,
}
