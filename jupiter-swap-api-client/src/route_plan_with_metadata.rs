use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::serde_helpers::field_as_string;

/// Topologically sorted DAG with additional metadata for rendering
pub type RoutePlanWithMetadata = Vec<RoutePlanStep>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanStep {
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwapInfo {
    #[serde(with = "field_as_string")]
    pub amm_key: Pubkey,
    pub label: String,
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// An estimation of the input amount into the AMM
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    /// An estimation of the output amount into the AMM
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// Fee amount (optional - not always returned by Jupiter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<String>,
    /// Fee token mint (optional - not always returned by Jupiter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_mint: Option<String>,
    /// Output amount after slippage is applied
    #[serde(with = "field_as_string")]
    pub out_amount_after_slippage: u64,
}
