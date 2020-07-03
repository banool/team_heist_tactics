use serde::{Deserialize, Deserializer};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// You can use this with serde to make it that empty strings inside Options become None.
// See https://github.com/serde-rs/serde/issues/1425 for more.
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}
