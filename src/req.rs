use std::{ops::Deref, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    Client, Method, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use crate::ExtraArgs;

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
    pub fn generate(&self, args: &ExtraArgs) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| json!({}));

        for (k, v) in &args.headers {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        if !headers.contains_key(header::CONTENT_TYPE) {
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        for (k, v) in &args.query {
            query[k] = v.parse()?;
        }
        for (k, v) in &args.body {
            body[k] = v.parse()?;
        }

        // application/json; charset=utf-8
        let content_type = headers
            .get(header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap().split(';').next())
            .flatten();

        let body: Result<String> = match content_type {
            Some("application/json") => Ok(serde_json::to_string(&body)?),
            Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
                Ok(serde_urlencoded::to_string(&body)?)
            }
            _ => Err(anyhow!("unsupported content-type")),
        };
        body.and_then(|body| Ok((headers, query, body)))
        // Ok((headers, query, body?))
    }

    pub async fn send(&self, args: &ExtraArgs) -> Result<ResponseExt> {
        // 先合并 requestProfile 和 ExtraArgs

        let (headers, query, body) = self.generate(args)?;

        let client = Client::new();

        let req = client
            .request(self.method.clone(), self.url.clone())
            .query(&query)
            .headers(headers)
            .body(body)
            .build()?;

        let res = client.execute(req).await?;
        Ok(ResponseExt(res))
    }
}

#[derive(Debug)]
pub struct ResponseExt(Response);

impl Deref for ResponseExt {
    type Target = Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
