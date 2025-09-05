use serde::{Serializer, Deserializer, Deserialize};

pub fn serialize<S>(vec: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match vec {
        Some(v) if !v.is_empty() => {
            let joined = v.join(",");
            serializer.serialize_str(&joined)
        }
        _ => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_string: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt_string.map(|s| s.split(',').map(|s| s.to_string()).collect()))
}