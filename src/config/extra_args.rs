use std::str::SplitN;
use std::{ops::Deref, str::FromStr};

use anyhow::{anyhow, Error};

#[derive(Debug, Clone)]
pub struct Parameter(Vec<(String, String)>);

impl From<Vec<KeyValue>> for Parameter {
    fn from(args: Vec<KeyValue>) -> Self {
        Self(args.into_iter().map(|v| (v.key, v.value)).collect())
    }
}
impl Deref for Parameter {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ExtraArgs {
    pub headers: Parameter,
    pub query: Parameter,
    pub body: Parameter,
}

// header query body 的 key: value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl FromStr for KeyValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '=');
        let get_key_or_value = |parts: &mut SplitN<char>| -> Result<String, anyhow::Error> {
            let s = parts
                .next()
                .ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
                .trim()
                .to_owned();
            Ok(s)
        };
        Ok(KeyValue {
            key: get_key_or_value(&mut parts)?,
            value: get_key_or_value(&mut parts)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_value_from_str() {
        let s = "a=b";

        let kv: KeyValue = s.parse().unwrap();
        assert_eq!(
            kv,
            KeyValue {
                key: "a".to_owned(),
                value: "b".to_owned(),
            }
        )
    }
}
