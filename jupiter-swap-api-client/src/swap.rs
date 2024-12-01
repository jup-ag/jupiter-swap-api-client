use crate::{
    quote::QuoteResponse, 
    serde_helpers::{
        field_as_base64, field_as_string, vec_field_as_pubkey, vec_field_as_instruction,
        option_field_as_instruction, field_as_instruction
    }, 
    transaction_config::TransactionConfig,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapRequest {
    #[serde(with = "field_as_string")]
    pub user_public_key: Pubkey,
    pub quote_response: QuoteResponse,
    #[serde(flatten)]
    pub config: TransactionConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PrioritizationType {
    #[serde(rename_all = "camelCase")]
    Jito { lamports: u64 },
    #[serde(rename_all = "camelCase")]
    ComputeBudget {
        micro_lamports: u64,
        estimated_micro_lamports: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DynamicSlippageReport {
    pub slippage_bps: u16,
    pub other_amount: Option<u64>,
    /// Signed to convey positive and negative slippage
    pub simulated_incurred_slippage_bps: Option<i16>,
    pub amplification_ratio: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UiSimulationError {
    error_code: String,
    error: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    #[serde(with = "field_as_base64")]
    pub swap_transaction: Vec<u8>,
    pub last_valid_block_height: u64,
    pub prioritization_fee_lamports: u64,
    pub compute_unit_limit: u32,
    pub prioritization_type: Option<PrioritizationType>,
    pub dynamic_slippage_report: Option<DynamicSlippageReport>,
    pub simulation_error: Option<UiSimulationError>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwapInstructionsResponse {
    #[serde(with = "option_field_as_instruction")]
    pub token_ledger_instruction: Option<Instruction>,
    #[serde(with = "vec_field_as_instruction")]
    pub compute_budget_instructions: Vec<Instruction>,
    #[serde(with = "vec_field_as_instruction")]
    pub setup_instructions: Vec<Instruction>,
    /// Instruction performing the action of swapping
    #[serde(with = "field_as_instruction")]
    pub swap_instruction: Instruction,
    #[serde(with = "option_field_as_instruction")]
    pub cleanup_instruction: Option<Instruction>,
    /// Other instructions that should be included in the transaction.
    /// Now, it should only have the Jito tip instruction.
    #[serde(with = "vec_field_as_instruction")]
    pub other_instructions: Vec<Instruction>,
    #[serde(with = "vec_field_as_pubkey")]
    pub address_lookup_table_addresses: Vec<Pubkey>,
    pub prioritization_fee_lamports: u64,
    pub compute_unit_limit: u32,
    pub prioritization_type: Option<PrioritizationType>,
    pub dynamic_slippage_report: Option<DynamicSlippageReport>,
    pub simulation_error: Option<UiSimulationError>,
}
