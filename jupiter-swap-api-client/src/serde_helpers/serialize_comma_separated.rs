use serde::Serializer;

// Custom serializer for a Vec<String> that outputs a single string with comma-separated values
pub fn serialize_comma_separated<S>(
    items: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    match items {
        Some(values) => {
            let joined = values.join(",");
            serializer.serialize_str(&joined)
        }
        None => serializer.serialize_none(),
    }
}
