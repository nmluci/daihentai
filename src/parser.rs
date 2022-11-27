use serde::{Deserialize, Deserializer};

pub fn parse_int(s: &String) -> i64 {
   return s.parse().unwrap()
}

pub fn deserialize_nullable<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}