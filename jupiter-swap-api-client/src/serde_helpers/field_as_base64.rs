use {
    serde::{Deserializer, Serializer},
    serde::{Deserialize, Serialize},
};
use base64::{engine::general_purpose::STANDARD, Engine};

pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
    String::serialize(&STANDARD.encode(v), s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    STANDARD
        .decode(String::deserialize(deserializer)?)
        .map_err(|e| serde::de::Error::custom(format!("base64 decoding error {:?}", e)))
}

