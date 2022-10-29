mod diff_config;
mod diff_profile;
mod extra_args;
mod profile;

use std::{ops::Deref, str::FromStr};

pub use diff_config::DiffConfig;
pub use diff_profile::DiffProfile;
pub use extra_args::{ExtraArgs, KeyVal, Parameter};
pub use profile::{RequestProfile, ResponseProfile};

use anyhow::{anyhow, Result};
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use serde_json::json;

pub fn generate(
    rp: &RequestProfile,
    args: &ExtraArgs,
) -> Result<(HeaderMap, serde_json::Value, String)> {
    let mut headers = rp.headers.clone();
    let mut query = rp.params.clone().unwrap_or_else(|| json!({}));
    let mut body = rp.body.clone().unwrap_or_else(|| json!({}));

    for (k, v) in args.headers.deref() {
        headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
    }

    if !headers.contains_key(header::CONTENT_TYPE) {
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
    }

    for (k, v) in args.query.deref() {
        query[k] = v.parse()?;
    }
    for (k, v) in args.body.deref() {
        body[k] = v.parse()?;
    }

    // application/json; charset=utf-8
    let content_type = get_content_type(&headers);

    let body: Result<String> = match content_type.as_deref() {
        Some("application/json") => Ok(serde_json::to_string(&body)?),
        Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
            Ok(serde_urlencoded::to_string(&body)?)
        }
        _ => Err(anyhow!("unsupported content-type")),
    };
    body.map(|body| (headers, query, body))
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().unwrap().split(';').next().map(|v| v.to_string()))
}
