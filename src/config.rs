use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::diff_text;
use crate::ExtraArgs;
use crate::RequestProfile;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub res: Option<ResponseProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl ResponseProfile {
    pub fn new(skip_headers: Vec<String>, skip_body: Vec<String>) -> Self {
        Self {
            skip_headers,
            skip_body,
        }
    }
}

impl DiffConfig {
    pub fn new(name: &str, profile: DiffProfile) -> Self {
        let mut m = HashMap::new();
        m.insert(name.to_owned(), profile);
        Self { profiles: m }
    }

    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl DiffProfile {
    pub fn new(req1: RequestProfile, req2: RequestProfile, res: Option<ResponseProfile>) -> Self {
        Self { req1, req2, res }
    }

    pub async fn diff(&self, args: ExtraArgs) -> Result<String> {
        let res1 = self.req1.send(&args).await?;
        let res2 = self.req2.send(&args).await?;

        let text1 = res1.get_text(&self.res).await?;
        let text2 = res2.get_text(&self.res).await?;

        diff_text(&text1, &text2)
    }
}
