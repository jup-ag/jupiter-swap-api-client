use {
    serde::{de, Deserialize, Deserializer, Serialize, Serializer},
    std::str::FromStr,
};

pub fn serialize<T, S>(t: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    if let Some(t) = t {
        t.to_string().serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    D: Deserializer<'de>,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => s
            .parse()
            .map(Some)
            .map_err(|e| de::Error::custom(format!("Parse error: {:?}", e))),
        None => Ok(None),
    }
}
