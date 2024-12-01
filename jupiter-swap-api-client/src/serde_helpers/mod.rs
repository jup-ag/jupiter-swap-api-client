use serde::{Deserialize, Deserializer};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;

pub mod field_as_string;
pub mod option_field_as_string;
pub mod field_as_base64;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstructionHelper {
    #[serde(with = "field_as_string")]
    program_id: Pubkey,
    accounts: Vec<AccountMetaHelper>,
    #[serde(with = "field_as_base64")]
    data: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountMetaHelper {
    #[serde(with = "field_as_string")]
    pubkey: Pubkey,
    is_signer: bool,
    is_writable: bool,
}

impl From<InstructionHelper> for Instruction {
    fn from(helper: InstructionHelper) -> Self {
        Instruction {
            program_id: helper.program_id,
            accounts: helper
                .accounts
                .into_iter()
                .map(|acc| AccountMeta {
                    pubkey: acc.pubkey,
                    is_signer: acc.is_signer,
                    is_writable: acc.is_writable,
                })
                .collect(),
            data: helper.data,
        }
    }
}

pub mod field_as_instruction {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instruction, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(InstructionHelper::deserialize(deserializer)?.into())
    }
}

pub mod option_field_as_instruction {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Instruction>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<InstructionHelper>::deserialize(deserializer)?.map(Into::into))
    }
}

pub mod vec_field_as_instruction {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Instruction>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<InstructionHelper>::deserialize(deserializer)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

pub mod vec_field_as_pubkey {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Pubkey>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "field_as_string")] Pubkey);

        Ok(Vec::<Helper>::deserialize(deserializer)?
            .into_iter()
            .map(|v| v.0)
            .collect())
    }
}
