use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::Profile;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, Profile>,
}

impl DiffConfig {
    pub fn new(name: &str, profile: Profile) -> Self {
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

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }
}
