use crate::{
    quote::QuoteResponse, serde_helpers::field_as_string, transaction_config::TransactionConfig,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
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
    #[serde(with = "base64_serialize_deserialize")]
    pub swap_transaction: Vec<u8>,
    pub last_valid_block_height: u64,
    pub prioritization_fee_lamports: u64,
    pub compute_unit_limit: u32,
    pub prioritization_type: Option<PrioritizationType>,
    pub dynamic_slippage_report: Option<DynamicSlippageReport>,
    pub simulation_error: Option<UiSimulationError>,
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
pub struct SwapInstructionsResponse {
    pub token_ledger_instruction: Option<Instruction>,
    pub compute_budget_instructions: Vec<Instruction>,
    pub setup_instructions: Vec<Instruction>,
    /// Instruction performing the action of swapping
    pub swap_instruction: Instruction,
    pub cleanup_instruction: Option<Instruction>,
    /// Other instructions that should be included in the transaction.
    /// Now, it should only have the Jito tip instruction.
    pub other_instructions: Vec<Instruction>,
    pub address_lookup_table_addresses: Vec<Pubkey>,
    pub prioritization_fee_lamports: u64,
    pub compute_unit_limit: u32,
    pub prioritization_type: Option<PrioritizationType>,
    pub dynamic_slippage_report: Option<DynamicSlippageReport>,
    pub simulation_error: Option<UiSimulationError>,
}

// Duplicate for deserialization
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwapInstructionsResponseInternal {
    token_ledger_instruction: Option<InstructionInternal>,
    compute_budget_instructions: Vec<InstructionInternal>,
    setup_instructions: Vec<InstructionInternal>,
    /// Instruction performing the action of swapping
    swap_instruction: InstructionInternal,
    cleanup_instruction: Option<InstructionInternal>,
    /// Other instructions that should be included in the transaction.
    /// Now, it should only have the Jito tip instruction.
    other_instructions: Vec<InstructionInternal>,
    address_lookup_table_addresses: Vec<PubkeyInternal>,
    prioritization_fee_lamports: u64,
    compute_unit_limit: u32,
    prioritization_type: Option<PrioritizationType>,
    dynamic_slippage_report: Option<DynamicSlippageReport>,
    simulation_error: Option<UiSimulationError>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct InstructionInternal {
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMetaInternal>,
    #[serde(with = "base64_serialize_deserialize")]
    pub data: Vec<u8>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct PubkeyInternal(#[serde(with = "field_as_string")] Pubkey);

impl From<InstructionInternal> for Instruction {
    fn from(val: InstructionInternal) -> Self {
        Instruction {
            program_id: val.program_id,
            accounts: val.accounts.into_iter().map(Into::into).collect(),
            data: val.data,
        }
    }
}

impl From<SwapInstructionsResponseInternal> for SwapInstructionsResponse {
    fn from(value: SwapInstructionsResponseInternal) -> Self {
        Self {
            token_ledger_instruction: value.token_ledger_instruction.map(Into::into),
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
            cleanup_instruction: value.cleanup_instruction.map(Into::into),
            other_instructions: value
                .other_instructions
                .into_iter()
                .map(Into::into)
                .collect(),
            address_lookup_table_addresses: value
                .address_lookup_table_addresses
                .into_iter()
                .map(|p| p.0)
                .collect(),
            prioritization_fee_lamports: value.prioritization_fee_lamports,
            compute_unit_limit: value.compute_unit_limit,
            prioritization_type: value.prioritization_type,
            dynamic_slippage_report: value.dynamic_slippage_report,
            simulation_error: value.simulation_error,
        }
    }
}
