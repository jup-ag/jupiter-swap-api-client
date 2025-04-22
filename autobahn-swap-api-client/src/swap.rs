use crate::{
    quote::QuoteResponse, serde_helpers::field_as_string,
};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use crate::serde_helpers::option_field_as_string;
use std::str::FromStr;
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapRequest {
    #[serde(with = "field_as_string")]
    pub user_public_key: String,
    pub wrap_and_unwrap_sol: bool,
    pub auto_create_out_ata: bool,
    pub use_shared_accounts: bool,
    #[serde(with = "option_field_as_string")]
    pub fee_account: Option<String>,
    pub compute_unit_price_micro_lamports: Option<u64>,
    pub as_legacy_transaction: bool,
    pub use_token_ledger: bool,
    #[serde(with = "option_field_as_string")]
    pub destination_token_account: Option<String>,
    pub quote_response: QuoteResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_as]
pub struct SwapResponse {
    #[serde_as(as = "Base64")]
    pub swap_transaction: Vec<u8>,
    pub last_valid_block_height: u64,
    pub priorization_fee_lamports: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwapIxResponse {
    pub token_ledger_instruction: Option<InstructionResponse>,
    pub compute_budget_instructions: Option<Vec<InstructionResponse>>,
    pub setup_instructions: Option<Vec<InstructionResponse>>,
    pub swap_instruction: InstructionResponse,
    pub cleanup_instructions: Option<Vec<InstructionResponse>>,
    pub address_lookup_table_addresses: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstructionResponse {
    pub program_id: String,
    pub data: Option<String>,
    pub accounts: Option<Vec<AccountMeta>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountMeta {
    pub pubkey: String,
    pub is_signer: Option<bool>,
    pub is_writable: Option<bool>,
}

impl InstructionResponse {
    pub fn from_ix(instruction: solana_sdk::instruction::Instruction) -> anyhow::Result<InstructionResponse> {
        Ok(Self {
            program_id: instruction.program_id.to_string(),
            data: Some(base64::encode(&instruction.data)),
            accounts: Some(
                instruction
                    .accounts
                    .into_iter()
                    .map(|x| AccountMeta {
                        pubkey: x.pubkey.to_string(),
                        is_signer: Some(x.is_signer),
                        is_writable: Some(x.is_writable),
                    })
                    .collect(),
            ),
        })
    }

    pub fn to_ix(&self) -> anyhow::Result<solana_sdk::instruction::Instruction> {
        self.try_into()
    }
}

impl TryFrom<&InstructionResponse> for solana_sdk::instruction::Instruction {
    type Error = anyhow::Error;
    fn try_from(m: &InstructionResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            program_id: Pubkey::from_str(&m.program_id)?,
            data: match m.data.as_ref() {
                Some(d) => base64::decode(d).map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))?,
                None => vec![],
            },
            accounts: match m.accounts.as_ref() {
                Some(accs) => accs
                    .iter()
                    .map(|a| a.try_into())
                    .collect::<anyhow::Result<Vec<solana_sdk::instruction::AccountMeta>>>()?,
                None => vec![],
            },
        })
    }
}

impl TryFrom<&AccountMeta> for solana_sdk::instruction::AccountMeta {
    type Error = anyhow::Error;
    fn try_from(m: &AccountMeta) -> Result<Self, Self::Error> {
        Ok(Self {
            pubkey: Pubkey::from_str(&m.pubkey)?,
            is_signer: m.is_signer.unwrap_or(false),
            is_writable: m.is_writable.unwrap_or(false),
        })
    }
}
