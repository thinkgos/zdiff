use std::{ops::Deref, str::FromStr};

use anyhow::{anyhow, Error};

#[derive(Debug, Clone)]
pub struct ExtraArgs {
    pub headers: Parameter,
    pub query: Parameter,
    pub body: Parameter,
}

#[derive(Debug, Clone)]
pub struct Parameter(Vec<(String, String)>);

impl From<Vec<KeyVal>> for Parameter {
    fn from(args: Vec<KeyVal>) -> Self {
        Self(args.into_iter().map(|v| (v.key, v.value)).collect())
    }
}

impl Deref for Parameter {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}

impl FromStr for KeyVal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '=');

        let key = parts
            .next()
            .ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
            .trim();
        let value = parts
            .next()
            .ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
            .trim();
        Ok(KeyVal {
            key: key.to_owned(),
            value: value.to_owned(),
        })
    }
}
