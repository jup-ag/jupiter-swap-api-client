use {serde::Serialize, serde::Serializer};

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
