//! Quote data structures for requesting a swap price and handling the response.
//! This is typically used by a DeFi routing or aggregation service on Solana.

use std::{collections::HashMap, str::FromStr};

use crate::route_plan_with_metadata::RoutePlanWithMetadata;
use crate::serde_helpers::field_as_string;
use anyhow::{anyhow, Error};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// --- Utility Type ---

/// Comma-delimited list of Decentralized Exchange (DEX) labels (e.g., "Raydium,Orca").
type Dexes = String;

// --- Swap Information Structure ---

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
/// Swap details for a single step in a multi-hop route.
pub struct SwapInfo {
    /// The PublicKey of the Automated Market Maker (AMM) pool or program.
    #[serde(with = "field_as_string")]
    pub amm_key: Pubkey,
    /// The human-readable label for the DEX/AMM (e.g., "Raydium_V4").
    pub label: String,
    /// The input token mint for this specific swap step.
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The output token mint for this specific swap step.
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// Estimated input amount into the AMM pool (factoring in token decimals).
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    /// Estimated output amount from the AMM pool (factoring in token decimals).
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
}

// --- Swap Mode Enumeration ---

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
/// Defines the direction of the swap, based on which amount is fixed.
pub enum SwapMode {
    /// The input amount is fixed; slippage occurs on the output amount. (Default)
    #[default]
    ExactIn,
    /// The output amount is fixed (e.g., for payments); slippage occurs on the input amount.
    ExactOut,
}

impl FromStr for SwapMode {
    type Err = Error;

    /// Attempts to convert a string slice into a SwapMode enum.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ExactIn" => Ok(Self::ExactIn),
            "ExactOut" => Ok(Self::ExactOut),
            _ => Err(anyhow!("'{}' is not a valid SwapMode. Expected 'ExactIn' or 'ExactOut'.", s)),
        }
    }
}

// --- Request Sub-Structures ---

#[derive(Serialize, Debug, Clone, Default)]
/// Represents scoring configuration based on Transaction Compute Units (CUs).
pub struct ComputeUnitScore {
    /// Maximum penalty (in basis points) applied to a route for high CU usage.
    pub max_penalty_bps: Option<f64>,
}

// --- Main Request Structures ---

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Full request payload sent by the client to obtain a swap quote and route plan.
pub struct QuoteRequest {
    /// The mint of the token being swapped (given).
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount of the input or output token (depending on `swap_mode`), factoring in token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// The swap direction (ExactIn or ExactOut). Defaults to ExactIn.
    pub swap_mode: Option<SwapMode>,
    /// The maximum allowed price slippage, measured in basis points (e.g., 50 for 0.5%).
    pub slippage_bps: u16,
    /// If true, the API suggests a dynamic 'smart' slippage. Defaults to false.
    pub auto_slippage: Option<bool>,
    /// The absolute upper limit for auto-slippage calculation (in basis points).
    pub max_auto_slippage_bps: Option<u16>,
    /// Enables or disables the computation of auto slippage.
    pub compute_auto_slippage: bool,
    /// The USD value collision threshold for auto slippage calculation.
    pub auto_slippage_collision_usd_value: Option<u32>,
    /// If true, the router tries a greater input amount to find a route that minimizes the effective slippage.
    pub minimize_slippage: Option<bool>,
    /// Optional platform fee to be collected (in basis points).
    pub platform_fee_bps: Option<u8>,
    /// A comma-separated list of DEXes to explicitly include in the search.
    pub dexes: Option<Dexes>,
    /// A comma-separated list of DEXes to explicitly exclude from the search.
    pub excluded_dexes: Option<Dexes>,
    /// If true, restricts routing to only direct token pair swaps (no multi-hop).
    pub only_direct_routes: Option<bool>,
    /// If true, the resulting transaction will attempt to fit into a legacy (non-versioned) transaction format.
    pub as_legacy_transaction: Option<bool>,
    /// Restricts intermediate tokens to a list known to have stable liquidity.
    pub restrict_intermediate_tokens: Option<bool>,
    /// Estimates and restricts the route to fit within a max number of accounts involved. Use with caution.
    pub max_accounts: Option<usize>,
    /// Identifier for the routing algorithm to be used.
    pub quote_type: Option<String>,
    /// Extra parameters specific to the chosen quote_type algorithm.
    pub quote_args: Option<HashMap<String, String>>,
    /// If true, favors DEXes that are fully liquid when selecting intermediate tokens.
    pub prefer_liquid_dexes: Option<bool>,
    /// Configuration for routing based on transaction compute unit score.
    pub compute_unit_score: Option<ComputeUnitScore>,
    /// Custom string constraints passed to the router (implementation-specific).
    pub routing_constraints: Option<String>,
    /// If true, uses token category information (e.g., stablecoin, wrapped asset) for intermediate token selection.
    pub token_category_based_intermediate_tokens: Option<bool>,
}

// Implement Default manually to provide a safer default slippage_bps.
impl Default for QuoteRequest {
    fn default() -> Self {
        QuoteRequest {
            // Standard default fields
            input_mint: Pubkey::default(),
            output_mint: Pubkey::default(),
            amount: 0,
            swap_mode: None,
            // Recommended default slippage for safe operation (0.5% or 50 BPS).
            slippage_bps: 50, 
            auto_slippage: None,
            max_auto_slippage_bps: None,
            compute_auto_slippage: false,
            auto_slippage_collision_usd_value: None,
            minimize_slippage: None,
            platform_fee_bps: None,
            dexes: None,
            excluded_dexes: None,
            only_direct_routes: None,
            as_legacy_transaction: None,
            restrict_intermediate_tokens: None,
            max_accounts: None,
            quote_type: None,
            prefer_liquid_dexes: None,
            compute_unit_score: None,
            routing_constraints: None,
            token_category_based_intermediate_tokens: None,
            // QuoteRequest specific fields
            quote_args: None,
        }
    }
}


#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Internal structure used by the routing engine, excluding fields unnecessary for the core logic.
/// This structure is derived from `QuoteRequest` but omits external/extra configuration fields.
pub struct InternalQuoteRequest {
    /// The mint of the token being swapped (given).
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount to swap, factoring in the token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// The swap direction (ExactIn or ExactOut).
    pub swap_mode: Option<SwapMode>,
    /// Allowed slippage in basis points.
    pub slippage_bps: u16,
    /// If true, the API will suggest smart slippage.
    pub auto_slippage: Option<bool>,
    /// The max amount of slippage in basis points for auto slippage.
    pub max_auto_slippage_bps: Option<u16>,
    /// Enables or disables the computation of auto slippage.
    pub compute_auto_slippage: bool,
    /// The max USD value collision threshold for auto slippage.
    pub auto_slippage_collision_usd_value: Option<u32>,
    /// If true, the router tries to minimize slippage.
    pub minimize_slippage: Option<bool>,
    /// Platform fee in basis points.
    pub platform_fee_bps: Option<u8>,
    /// DEXes explicitly included in the search.
    pub dexes: Option<Dexes>,
    /// DEXes explicitly excluded from the search.
    pub excluded_dexes: Option<Dexes>,
    /// If true, only direct token routes are considered.
    pub only_direct_routes: Option<bool>,
    /// If true, attempts to fit the quote into a legacy transaction.
    pub as_legacy_transaction: Option<bool>,
    /// Restricts intermediate tokens to a safe, liquid set.
    pub restrict_intermediate_tokens: Option<bool>,
    /// Maximum estimated number of accounts involved in the route.
    pub max_accounts: Option<usize>,
    /// Identifier for the routing algorithm.
    pub quote_type: Option<String>,
    /// If true, enables only liquid markets as intermediate tokens.
    pub prefer_liquid_dexes: Option<bool>,
}

impl From<QuoteRequest> for InternalQuoteRequest {
    /// Converts a client's QuoteRequest into the simplified InternalQuoteRequest used for core routing.
    fn from(request: QuoteRequest) -> Self {
        InternalQuoteRequest {
            // Fields are explicitly mapped, dropping request.quote_args and other specific fields.
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
            excluded_dexes: request.excluded_dexes,
            only_direct_routes: request.only_direct_routes,
            as_legacy_transaction: request.as_legacy_transaction,
            restrict_intermediate_tokens: request.restrict_intermediate_tokens,
            max_accounts: request.max_accounts,
            quote_type: request.quote_type,
            prefer_liquid_dexes: request.prefer_liquid_dexes,
        }
    }
}

// --- Response Sub-Structure ---

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// Details about the platform fee collected for the swap.
pub struct PlatformFee {
    /// The fee amount collected (factoring in token decimals).
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// The fee percentage collected, in basis points (BPS).
    pub fee_bps: u8,
}

// --- Main Response Structure ---

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
/// The final response containing the best quote and the path to execute the swap.
pub struct QuoteResponse {
    /// The mint of the token provided by the user.
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The final input amount needed for the route (may differ slightly if SwapMode::ExactOut).
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    /// The mint of the token to be received by the user.
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The final output amount expected from the route (may differ slightly if SwapMode::ExactIn).
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// The threshold amount on the non-fixed side of the swap. Used for validation/slippage.
    /// (e.g., minimum out for ExactIn, maximum in for ExactOut).
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    /// The mode used for calculating the quote (ExactIn or ExactOut).
    pub swap_mode: SwapMode,
    /// The slippage basis points used for the quote calculation.
    pub slippage_bps: u16,
    /// The dynamically computed slippage used, if auto-slippage was enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_auto_slippage: Option<u16>,
    /// Indicates if the quote minimized slippage by changing the input amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_quote_minimizing_slippage: Option<bool>,
    /// Details on the platform fee collected, if any.
    pub platform_fee: Option<PlatformFee>,
    /// The percentage impact the swap will have on the liquidity pool price.
    pub price_impact_pct: Decimal,
    /// The detailed list of steps (swaps) that make up the final route.
    pub route_plan: RoutePlanWithMetadata,
    /// The slot number of the Solana network at the time the quote was generated. (Default 0)
    #[serde(default)]
    pub context_slot: u64,
    /// The time taken (in seconds) to generate this quote. (Default 0.0)
    #[serde(default)]
    pub time_taken: f64,
}
