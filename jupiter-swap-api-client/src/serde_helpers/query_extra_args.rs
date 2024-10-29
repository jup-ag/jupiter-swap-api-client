use std::collections::HashMap;

use serde::{ser::SerializeMap, Serializer};

pub fn serialize_extra_args<S>(
    extra_args: &Option<HashMap<String, String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(args) = extra_args {
        let mut map = serializer.serialize_map(Some(args.len()))?;
        for (k, v) in args {
            map.serialize_entry(k, v)?;
        }
        map.end()
    } else {
        serializer.serialize_none()
    }
}