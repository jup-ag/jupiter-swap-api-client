//! Quote data structure for quoting and quote response
//!

use std::{collections::HashMap, str::FromStr};

use crate::route_plan_with_metadata::RoutePlanWithMetadata;
use crate::serde_helpers::field_as_string;
use anyhow::{anyhow, Error};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Swap information of each Swap occurred in the route paths
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
    #[serde(with = "field_as_string")]
    pub fee_amount: u64,
    #[serde(with = "field_as_string")]
    pub fee_mint: Pubkey,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
pub enum SwapMode {
    #[default]
    ExactIn,
    ExactOut,
}

impl FromStr for SwapMode {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ExactIn" => Ok(Self::ExactIn),
            "ExactOut" => Ok(Self::ExactOut),
            _ => Err(anyhow!("{} is not a valid SwapMode", s)),
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct ComputeUnitScore {
    pub max_penalty_bps: Option<f64>,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount to swap, have to factor in the token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// (ExactIn or ExactOut) Defaults to ExactIn.
    /// ExactOut is for supporting use cases where you need an exact token amount, like payments.
    /// In this case the slippage is on the input token.
    pub swap_mode: Option<SwapMode>,
    /// Allowed slippage in basis points
    pub slippage_bps: u16,
    /// Default is false.
    /// By setting this to true, our API will suggest smart slippage info that you can use.
    /// slippageBps is what we suggest you to use. Additionally, you should check out max_auto_slippage_bps and auto_slippage_collision_usd_value.
    pub auto_slippage: Option<bool>,
    /// The max amount of slippage in basis points that you are willing to accept for auto slippage.
    pub max_auto_slippage_bps: Option<u16>,
    pub compute_auto_slippage: bool,
    /// The max amount of USD value that you are willing to accept for auto slippage.
    pub auto_slippage_collision_usd_value: Option<u32>,
    /// Quote with a greater amount to find the route to minimize slippage
    pub minimize_slippage: Option<bool>,
    /// Platform fee in basis points
    pub platform_fee_bps: Option<u8>,
    pub dexes: Option<Dexes>,
    pub exclude_dexes: Option<Dexes>,
    /// Quote only direct routes
    pub only_direct_routes: Option<bool>,
    /// Quote fit into legacy transaction
    pub as_legacy_transaction: Option<bool>,
    /// Restrict intermediate tokens to a top token set that has stable liquidity.
    /// This will help to ease potential high slippage error rate when swapping with minimal impact on pricing.
    pub restrict_intermediate_tokens: Option<bool>,
    /// Find a route given a maximum number of accounts involved,
    /// this might dangerously limit routing ending up giving a bad price.
    /// The max is an estimation and not the exact count
    pub max_accounts: Option<usize>,
    /// Quote type to be used for routing, switches the algorithm
    pub quote_type: Option<String>,
    /// Extra args which are quote type specific to allow controlling settings from the top level
    pub quote_args: Option<HashMap<String, String>>,
    /// enable only full liquid markets as intermediate tokens
    pub prefer_liquid_dexes: Option<bool>,
    /// Use the compute unit score to pick a route
    pub compute_unit_score: Option<ComputeUnitScore>,
    /// Routing constraints
    pub routing_constraints: Option<String>,
    /// Token category based intermediates token
    pub token_category_based_intermediate_tokens: Option<bool>,
}

// Essentially the same as QuoteRequest, but without the extra args
// as we pass the extra args separately
#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InternalQuoteRequest {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount to swap, have to factor in the token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// (ExactIn or ExactOut) Defaults to ExactIn.
    /// ExactOut is for supporting use cases where you need an exact token amount, like payments.
    /// In this case the slippage is on the input token.
    pub swap_mode: Option<SwapMode>,
    /// Allowed slippage in basis points
    pub slippage_bps: u16,
    /// Default is false.
    /// By setting this to true, our API will suggest smart slippage info that you can use.
    /// slippageBps is what we suggest you to use. Additionally, you should check out max_auto_slippage_bps and auto_slippage_collision_usd_value.
    pub auto_slippage: Option<bool>,
    /// The max amount of slippage in basis points that you are willing to accept for auto slippage.
    pub max_auto_slippage_bps: Option<u16>,
    pub compute_auto_slippage: bool,
    /// The max amount of USD value that you are willing to accept for auto slippage.
    pub auto_slippage_collision_usd_value: Option<u32>,
    /// Quote with a greater amount to find the route to minimize slippage
    pub minimize_slippage: Option<bool>,
    /// Platform fee in basis points
    pub platform_fee_bps: Option<u8>,
    pub dexes: Option<Dexes>,
    pub exclude_dexes: Option<Dexes>,
    /// Quote only direct routes
    pub only_direct_routes: Option<bool>,
    /// Quote fit into legacy transaction
    pub as_legacy_transaction: Option<bool>,
    /// Restrict intermediate tokens to a top token set that has stable liquidity.
    /// This will help to ease potential high slippage error rate when swapping with minimal impact on pricing.
    pub restrict_intermediate_tokens: Option<bool>,
    /// Find a route given a maximum number of accounts involved,
    /// this might dangerously limit routing ending up giving a bad price.
    /// The max is an estimation and not the exact count
    pub max_accounts: Option<usize>,
    // Quote type to be used for routing, switches the algorithm
    pub quote_type: Option<String>,
    // enable only full liquid markets as intermediate tokens
    pub prefer_liquid_dexes: Option<bool>,
}

impl From<QuoteRequest> for InternalQuoteRequest {
    fn from(request: QuoteRequest) -> Self {
        InternalQuoteRequest {
            input_mint: request.input_mint,
            output_mint: request.output_mint,
            amount: request.amount,
            swap_mode: request.swap_mode,
            slippage_bps: request.slippage_bps,
            auto_slippage: request.auto_slippage,
            max_auto_slippage_bps: request.max_auto_slippage_bps,
            compute_auto_slippage: request.compute_auto_slippage,
            auto_slippage_collision_usd_value: request.auto_slippage_collision_usd_value,
            minimize_slippage: request.minimize_slippage,
            platform_fee_bps: request.platform_fee_bps,
            dexes: request.dexes,
            exclude_dexes: request.exclude_dexes,
            only_direct_routes: request.only_direct_routes,
            as_legacy_transaction: request.as_legacy_transaction,
            restrict_intermediate_tokens: request.restrict_intermediate_tokens,
            max_accounts: request.max_accounts,
            quote_type: request.quote_type,
            prefer_liquid_dexes: request.prefer_liquid_dexes,
        }
    }
}

/// Comma delimited list of dex labels
type Dexes = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFee {
    #[serde(with = "field_as_string")]
    pub amount: u64,
    pub fee_bps: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// Not used by build transaction
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    pub swap_mode: SwapMode,
    pub slippage_bps: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_auto_slippage: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_quote_minimizing_slippage: Option<bool>,
    pub platform_fee: Option<PlatformFee>,
    pub price_impact_pct: Decimal,
    pub route_plan: RoutePlanWithMetadata,
    #[serde(default)]
    pub context_slot: u64,
    #[serde(default)]
    pub time_taken: f64,
}
