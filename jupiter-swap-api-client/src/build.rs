//! Build data structures for requesting a swap price and handling the response.
//! This is typically used by a DeFi routing or aggregation service on Solana.

use std::fmt;

use crate::{route_plan_with_metadata::RoutePlanWithMetadata, serde_helpers::{field_as_string, option_field_as_string}};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

/// Comma-delimited list of Decentralized Exchange (DEX) labels (e.g., "Raydium,Orca").
type Dexes = String;

// --- Main Request Structures ---

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Full request payload sent by the client to obtain a swap quote and route plan.
pub struct BuildRequest {
    /// The mint of the token being swapped (given).
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount of the input or output token (depending on `swap_mode`), factoring in token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// Your wallet address    
    #[serde(with = "field_as_string")]
    pub taker: Pubkey,
    /// SOL tip amount in lamports. Adds a tip instruction for use with /submit
    #[serde(with = "option_field_as_string")]
    pub tip_amount: Option<u64>,
    /// Priority fee percentile for the CU price instruction. Named levels: "medium" (25th), "high" (50th), "veryHigh" (75th), or an integer 0-10000 in bps.
    pub compute_unit_price_percentile: Option<ComputeUnitPricePercentile>,
    /// “fast” for reduced latency routing (BETA)
    pub mode: Option<QuoteMode>,
    /// Maximum accounts for the swap route (1-64). Default: 64.
    pub max_accounts: Option<u8>,
    /// Integrator platform fee in bps (requires feeAccount)
    pub platform_fee_bps: Option<u16>,
    /// Token account to collect platform fees
    #[serde(with = "option_field_as_string")]
    pub fee_account: Option<Pubkey>,
    /// Account that pays transaction fees and rent. Defaults to taker when not passed in.
    #[serde(with = "option_field_as_string")]
    pub payer: Option<Pubkey>,
    /// Whether to wrap/unwrap SOL. Defaults to true.
    pub wrap_and_unwrap_sol: Option<bool>,
    /// Comma-separated list of DEXes to restrict routing to. Mutually exclusive with excludeDexes.
    pub dexes: Option<Dexes>,
    /// Comma-separated list of DEXes to exclude. Mutually exclusive with dexes.
    pub exclude_dexes: Option<Dexes>,
    /// SPL token account to receive output tokens. Mutually exclusive with nativeDestinationAccount.
    #[serde(with = "option_field_as_string")]
    pub destination_token_account: Option<Pubkey>,
    /// Native SOL account to receive output. Mutually exclusive with destinationTokenAccount.
    #[serde(with = "option_field_as_string")]
    pub native_destination_account: Option<Pubkey>,
    /// Number of slots until the blockhash expires. Defaults to 150.
    pub blockhash_slots_to_expiry: Option<u16>,
    /// Slippage tolerance in basis points. Defaults to 50.
    pub slippage_bps: Option<SlippageBps>
}

// Implement Default manually to provide a safer default slippage_bps.
impl Default for BuildRequest {
    fn default() -> Self {
        BuildRequest {
            // Standard default fields
            input_mint: Pubkey::default(),
            output_mint: Pubkey::default(),
            amount: 0,
            taker: Pubkey::default(),
            tip_amount: None,
            compute_unit_price_percentile: None,
            mode: None,
            max_accounts: None,
            platform_fee_bps: None,
            fee_account: None,
            payer: None,
            wrap_and_unwrap_sol: None,
            dexes: None,
            exclude_dexes: None,
            destination_token_account: None,
            native_destination_account: None,
            blockhash_slots_to_expiry: None,
            slippage_bps: None,
        }
    }
}


#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// Internal structure used by the routing engine, excluding fields unnecessary for the core logic.
/// This structure is derived from `BuildRequest` but omits external/extra configuration fields.
pub struct InternalBuildRequest {
    /// The mint of the token being swapped (given).
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// The amount of the input or output token (depending on `swap_mode`), factoring in token decimals.
    #[serde(with = "field_as_string")]
    pub amount: u64,
    /// Your wallet address    
    #[serde(with = "field_as_string")]
    pub taker: Pubkey,
    /// SOL tip amount in lamports. Adds a tip instruction for use with /submit
    #[serde(with = "option_field_as_string")]
    pub tip_amount: Option<u64>,
    /// Priority fee percentile for the CU price instruction. Named levels: "medium" (25th), "high" (50th), "veryHigh" (75th), or an integer 0-10000 in bps.
    pub compute_unit_price_percentile: Option<ComputeUnitPricePercentile>,
    /// “fast” for reduced latency routing (BETA)
    pub mode: Option<QuoteMode>,
    /// Maximum accounts for the swap route (1-64). Default: 64.
    pub max_accounts: Option<u8>,
    /// Integrator platform fee in bps (requires feeAccount)
    pub platform_fee_bps: Option<u16>,
    /// Token account to collect platform fees
    #[serde(with = "option_field_as_string")]
    pub fee_account: Option<Pubkey>,
    /// Account that pays transaction fees and rent. Defaults to taker when not passed in.
    #[serde(with = "option_field_as_string")]
    pub payer: Option<Pubkey>,
    /// Whether to wrap/unwrap SOL. Defaults to true.
    pub wrap_and_unwrap_sol: Option<bool>,
    /// Comma-separated list of DEXes to restrict routing to. Mutually exclusive with excludeDexes.
    pub dexes: Option<Dexes>,
    /// Comma-separated list of DEXes to exclude. Mutually exclusive with dexes.
    pub exclude_dexes: Option<Dexes>,
    /// SPL token account to receive output tokens. Mutually exclusive with nativeDestinationAccount.
    #[serde(with = "option_field_as_string")]
    pub destination_token_account: Option<Pubkey>,
    /// Native SOL account to receive output. Mutually exclusive with destinationTokenAccount.
    #[serde(with = "option_field_as_string")]
    pub native_destination_account: Option<Pubkey>,
    /// Number of slots until the blockhash expires. Defaults to 150.
    pub blockhash_slots_to_expiry: Option<u16>,
    /// Slippage tolerance in basis points. Defaults to 50.
    pub slippage_bps: Option<SlippageBps>
}

impl From<BuildRequest> for InternalBuildRequest {
    /// Converts a client's QuoteRequest into the simplified InternalQuoteRequest used for core routing.
    fn from(request: BuildRequest) -> Self {
        InternalBuildRequest {
            // Fields are explicitly mapped, dropping request.quote_args and other specific fields.
            input_mint: request.input_mint,
            output_mint: request.output_mint,
            amount: request.amount,
            slippage_bps: request.slippage_bps,
            platform_fee_bps: request.platform_fee_bps,
            dexes: request.dexes,
            max_accounts: request.max_accounts,
            taker: request.taker,
            tip_amount: request.tip_amount,
            compute_unit_price_percentile: request.compute_unit_price_percentile,
            mode: request.mode,
            fee_account: request.fee_account,
            payer: request.payer,
            wrap_and_unwrap_sol: request.wrap_and_unwrap_sol,
            exclude_dexes: request.exclude_dexes,
            destination_token_account: request.destination_token_account,
            native_destination_account: request.native_destination_account,
            blockhash_slots_to_expiry: request.blockhash_slots_to_expiry,
        }
    }
}

pub mod base64_serialize_deserialize {
  use base64::{engine::general_purpose::STANDARD, Engine};
  use serde::{de, Deserializer, Serializer};

  use super::*;
  pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
      let base58 = STANDARD.encode(v);
      String::serialize(&base58, s)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
  where
      D: Deserializer<'de>,
  {
      let field_string = String::deserialize(deserializer)?;
      STANDARD
          .decode(field_string)
          .map_err(|e| de::Error::custom(format!("base64 decoding error: {:?}", e)))
  }
}

#[derive(Debug, Clone)]
pub struct BuildInstructionsResponse {
    /// The mint of the token being swapped (given).
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    pub output_mint: Pubkey,
    pub in_amount: u64,
    pub out_amount: u64,
    /// Minimum output amount after slippage
    pub other_amount_threshold: u64,
    pub swap_mode: String,
    pub slippage_bps: u16,
    pub route_plan: RoutePlanWithMetadata,
    /// Compute unit price instruction (does not include compute unit limit)
    pub compute_budget_instructions: Vec<Instruction>,
    /// Pre-swap setup instructions (e.g. create ATAs)
    pub setup_instructions: Vec<Instruction>,
    pub swap_instruction: Instruction,
    /// Post-swap cleanup instruction
    pub cleanup_instruction: Instruction,
    pub other_instructions: Vec<Instruction>,
    pub tip_instruction: Instruction,
    pub addresses_by_lookup_table_address: Vec<Pubkey>
}

impl From<BuildInstructionsResponseInternal> for BuildInstructionsResponse {
  fn from(value: BuildInstructionsResponseInternal) -> Self {
      Self {
          compute_budget_instructions: value
              .compute_budget_instructions
              .into_iter()
              .map(Into::into)
              .collect(),
          setup_instructions: value
              .setup_instructions
              .into_iter()
              .map(Into::into)
              .collect(),
          swap_instruction: value.swap_instruction.into(),
          cleanup_instruction: value.cleanup_instruction.into(),
          other_instructions: value
              .other_instructions
              .into_iter()
              .map(Into::into)
              .collect(),
          addresses_by_lookup_table_address: value
              .addresses_by_lookup_table_address
              .into_iter()
              .map(|p| p.0)
              .collect(),
        tip_instruction: value.tip_instruction.into(),
        input_mint: value.input_mint,
        output_mint: value.output_mint,
        in_amount: value.in_amount,
        out_amount: value.out_amount,
        other_amount_threshold: value.other_amount_threshold,
        swap_mode: value.swap_mode,
        slippage_bps: value.slippage_bps,
        route_plan: value.route_plan,
      }
  }
}

// Duplicate for deserialization
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuildInstructionsResponseInternal {
    /// The mint of the token being swapped (given).
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    /// The mint of the token to be received (wanted).
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// Minimum output amount after slippage
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    pub swap_mode: String,
    pub slippage_bps: u16,
    pub route_plan: RoutePlanWithMetadata,
    /// Compute unit price instruction (does not include compute unit limit)
    pub compute_budget_instructions: Vec<InstructionInternal>,
    /// Pre-swap setup instructions (e.g. create ATAs)
    pub setup_instructions: Vec<InstructionInternal>,
    pub swap_instruction: InstructionInternal,
    /// Post-swap cleanup instruction
    pub cleanup_instruction: InstructionInternal,
    pub other_instructions: Vec<InstructionInternal>,
    pub tip_instruction: InstructionInternal,
    pub addresses_by_lookup_table_address: Vec<PubkeyInternal>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PubkeyInternal(#[serde(with = "field_as_string")] Pubkey);

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstructionInternal {
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMetaInternal>,
    #[serde(with = "base64_serialize_deserialize")]
    pub data: Vec<u8>,
}

impl From<InstructionInternal> for Instruction {
  fn from(val: InstructionInternal) -> Self {
      Instruction {
          program_id: val.program_id,
          accounts: val.accounts.into_iter().map(Into::into).collect(),
          data: val.data,
      }
  }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountMetaInternal {
    #[serde(with = "field_as_string")]
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<AccountMetaInternal> for AccountMeta {
    fn from(val: AccountMetaInternal) -> Self {
        AccountMeta {
            pubkey: val.pubkey,
            is_signer: val.is_signer,
            is_writable: val.is_writable,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComputeUnitPricePercentile {
    /// 25th percentile.
    Medium,
    /// 50th percentile.
    High,
    /// 75th percentile.
    VeryHigh,
    /// Arbitrary percentile in basis points (0–10 000).
    Bps(u16),
}

impl ComputeUnitPricePercentile {
    pub fn from_bps(bps: u16) -> Result<Self, &'static str> {
        if bps > 10_000 {
            Err("computeUnitPricePercentile bps must be 0–10 000")
        } else {
            Ok(Self::Bps(bps))
        }
    }
}

impl Serialize for ComputeUnitPricePercentile {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Medium   => s.serialize_str("medium"),
            Self::High     => s.serialize_str("high"),
            Self::VeryHigh => s.serialize_str("veryHigh"),
            Self::Bps(n)   => s.serialize_u16(*n),
        }
    }
}

impl<'de> Deserialize<'de> for ComputeUnitPricePercentile {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ComputeUnitPricePercentile;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(r#""medium", "high", "veryHigh", or an integer 0–10 000"#)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match v {
                    "medium"   => Ok(ComputeUnitPricePercentile::Medium),
                    "high"     => Ok(ComputeUnitPricePercentile::High),
                    "veryHigh" => Ok(ComputeUnitPricePercentile::VeryHigh),
                    other      => Err(E::unknown_variant(other, &["medium", "high", "veryHigh"])),
                }
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                let bps = u16::try_from(v)
                    .map_err(|_| E::custom(format!("{v} exceeds maximum bps of 10 000")))?;
                ComputeUnitPricePercentile::from_bps(bps)
                    .map_err(E::custom)
            }

            // JSON integers can arrive as i64 too
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                if v < 0 {
                    return Err(E::custom(format!("{v} is negative")));
                }
                self.visit_u64(v as u64)
            }
        }

        d.deserialize_any(Visitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum QuoteMode {
    /// Reduced latency routing at the cost of route optimality. Currently in BETA.
    #[serde(rename = "fast")]
    Fast,
}

#[derive(Debug, Clone)]
pub enum SlippageBps {
    /// Fixed slippage tolerance in basis points (0–10 000).
    Bps(u16),
    /// Real-time slippage estimation (RTSE).
    Rtse,
}

impl Default for SlippageBps {
    fn default() -> Self {
        Self::Bps(50)
    }
}

impl SlippageBps {
    pub fn from_bps(bps: u16) -> Result<Self, &'static str> {
        if bps > 10_000 {
            Err("slippageBps must be 0–10 000")
        } else {
            Ok(Self::Bps(bps))
        }
    }
}

impl Serialize for SlippageBps {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bps(n) => s.serialize_u16(*n),
            Self::Rtse   => s.serialize_str("rtse"),
        }
    }
}

impl<'de> Deserialize<'de> for SlippageBps {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SlippageBps;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(r#"an integer 0–10 000 or "rtse""#)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match v {
                    "rtse" => Ok(SlippageBps::Rtse),
                    other  => Err(E::unknown_variant(other, &["rtse"])),
                }
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                let bps = u16::try_from(v)
                    .map_err(|_| E::custom(format!("{v} exceeds maximum bps of 10 000")))?;
                SlippageBps::from_bps(bps).map_err(E::custom)
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                if v < 0 {
                    return Err(E::custom(format!("{v} is negative")));
                }
                self.visit_u64(v as u64)
            }
        }

        d.deserialize_any(Visitor)
    }
}