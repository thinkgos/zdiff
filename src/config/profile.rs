use std::str::FromStr;

use anyhow::{Ok, Result};
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,
    pub body: Option<serde_json::Value>,
}

impl FromStr for RequestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut url: Url = s.parse()?;
        let qs = url.query_pairs();
        let mut params = json!({});

        for (k, v) in qs {
            params[&*k] = v.parse()?;
        }
        url.set_query(None);

        // 如果json的object为空, 则为为None
        let params =
            if params.is_null() || (params.is_object() && params.as_object().unwrap().is_empty()) {
                None
            } else {
                Some(params)
            };

        Ok(RequestProfile {
            method: Method::GET,
            url,
            params,
            headers: HeaderMap::new(),
            body: None,
        })
    }
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
