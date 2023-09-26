use serde::{Deserialize, Deserializer, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::serde_helpers::option_field_as_string;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ComputeUnitPriceMicroLamports {
    MicroLamports(u64),
    #[serde(deserialize_with = "auto")]
    Auto,
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

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct TransactionConfig {
    /// Wrap and unwrap SOL. Will be ignored if `destination_token_account` is set because the `destination_token_account` may belong to a different user that we have no authority to close.
    pub wrap_and_unwrap_sol: bool,
    /// Fee token account for the output token, it is derived using the seeds = ["referral_ata", referral_account, mint] and the `REFER4ZgmyYx9c6He5XfaTMiGfdLwRnkV4RPp9t9iF3` referral contract (only pass in if you set a feeBps and make sure that the feeAccount has been created)
    #[serde(with = "option_field_as_string")]
    pub fee_account: Option<Pubkey>,
    /// Public key of the token account that will be used to receive the token out of the swap. If not provided, the user's ATA will be used. If provided, we assume that the token account is already initialized.
    #[serde(with = "option_field_as_string")]
    pub destination_token_account: Option<Pubkey>,
    /// compute unit price to prioritize the transaction, the additional fee will be compute unit consumed * computeUnitPriceMicroLamports
    pub compute_unit_price_micro_lamports: Option<ComputeUnitPriceMicroLamports>,
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
}

impl Default for TransactionConfig {
    fn default() -> Self {
        Self {
            wrap_and_unwrap_sol: true,
            fee_account: None,
            destination_token_account: None,
            compute_unit_price_micro_lamports: None,
            as_legacy_transaction: false,
            use_shared_accounts: true,
            use_token_ledger: false,
        }
    }
}
