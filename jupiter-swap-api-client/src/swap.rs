use crate::{
    quote::QuoteResponse, serde_helpers::field_as_string, transaction_config::TransactionConfig,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapRequest {
    #[serde(with = "field_as_string")]
    pub user_public_key: Pubkey,
    pub quote_response: QuoteResponse,
    #[serde(flatten)]
    pub config: TransactionConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    #[serde(with = "base64_deserialize")]
    pub swap_transaction: Vec<u8>,
    pub last_valid_block_height: u64,
}

mod base64_deserialize {
    use super::*;
    use serde::{de, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let swap_transaction_string = String::deserialize(deserializer)?;
        base64::decode(swap_transaction_string)
            .map_err(|e| de::Error::custom(format!("base64 decoding error: {:?}", e)))
    }
}

#[derive(Debug)]
pub struct SwapInstructionsResponse {
    pub token_ledger_instruction: Option<Instruction>,
    pub compute_budget_instructions: Vec<Instruction>,
    pub setup_instructions: Vec<Instruction>,
    /// Instruction performing the action of swapping
    pub swap_instruction: Instruction,
    pub cleanup_instruction: Option<Instruction>,
    pub address_lookup_table_addresses: Vec<Pubkey>,
}

// Duplicate for deserialization
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapInstructionsResponseInternal {
    token_ledger_instruction: Option<InstructionInternal>,
    compute_budget_instructions: Vec<InstructionInternal>,
    setup_instructions: Vec<InstructionInternal>,
    /// Instruction performing the action of swapping
    swap_instruction: InstructionInternal,
    cleanup_instruction: Option<InstructionInternal>,
    address_lookup_table_addresses: Vec<PubkeyInternal>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InstructionInternal {
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMetaInternal>,
    #[serde(with = "base64_deserialize")]
    pub data: Vec<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountMetaInternal {
    #[serde(with = "field_as_string")]
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl Into<AccountMeta> for AccountMetaInternal {
    fn into(self) -> AccountMeta {
        AccountMeta {
            pubkey: self.pubkey,
            is_signer: self.is_signer,
            is_writable: self.is_writable,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PubkeyInternal(#[serde(with = "field_as_string")] Pubkey);

impl Into<Instruction> for InstructionInternal {
    fn into(self) -> Instruction {
        Instruction {
            program_id: self.program_id,
            accounts: self.accounts.into_iter().map(Into::into).collect(),
            data: self.data,
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
            address_lookup_table_addresses: value
                .address_lookup_table_addresses
                .into_iter()
                .map(|p| p.0)
                .collect(),
        }
    }
}
