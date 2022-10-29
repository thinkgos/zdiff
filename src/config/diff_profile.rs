use serde::{Deserialize, Serialize};

use super::{RequestProfile, ResponseProfile};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub res: Option<ResponseProfile>,
}
impl DiffProfile {
    pub fn new(req1: RequestProfile, req2: RequestProfile, res: Option<ResponseProfile>) -> Self {
        Self { req1, req2, res }
    }
}
