use std::ops::Deref;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use mime::Mime;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
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

impl RequestProfile {
    pub fn merge(&self, args: &super::ExtraArgs) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| json!({}));

        for (k, v) in args.headers.deref() {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }
        for (k, v) in args.query.deref() {
            query[k] = v.parse()?;
        }
        for (k, v) in args.body.deref() {
            body[k] = v.parse()?;
        }
        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        // application/json; charset=utf-8
        let content_type = get_content_type(&headers);

        let body: Result<String> = match content_type {
            Some(v) if v == mime::APPLICATION_JSON => Ok(serde_json::to_string(&body)?),
            Some(v)
                if v == mime::APPLICATION_WWW_FORM_URLENCODED || v == mime::MULTIPART_FORM_DATA =>
            {
                Ok(serde_urlencoded::to_string(&body)?)
            }
            _ => Err(anyhow!("unsupported content-type")),
        };
        body.map(|body| (headers, query, body))
    }
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub res: Option<ResponseProfile>,
}

impl Profile {
    pub fn new(req1: RequestProfile, req2: RequestProfile, res: Option<ResponseProfile>) -> Self {
        Self { req1, req2, res }
    }
}

fn get_content_type(headers: &HeaderMap) -> Option<Mime> {
    headers
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}
