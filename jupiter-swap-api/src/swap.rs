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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapInstructionsResponse {
    token_ledger_instruction: Option<SerializableInstruction>,
    compute_budget_instructions: Vec<SerializableInstruction>,
    setup_instructions: Vec<SerializableInstruction>,
    /// Instruction performing the action of swapping
    swap_instruction: SerializableInstruction,
    cleanup_instruction: Option<SerializableInstruction>,
    address_lookup_table_addresses: Vec<SerializablePubkey>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SerializableInstruction {
    #[serde(with = "field_as_string")]
    pub program_id: Pubkey,
    pub accounts: Vec<SerializableAccountMeta>,
    pub data: Vec<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(remote = "AccountMeta")]
struct AccountMetaResponse {
    #[serde(with = "field_as_string")]
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SerializableAccountMeta(#[serde(with = "AccountMetaResponse")] AccountMeta);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SerializablePubkey(#[serde(with = "field_as_string")] Pubkey);

impl From<Instruction> for SerializableInstruction {
    fn from(value: Instruction) -> Self {
        Self {
            program_id: value.program_id,
            accounts: value
                .accounts
                .into_iter()
                .map(SerializableAccountMeta)
                .collect(),
            data: value.data,
        }
    }
}
