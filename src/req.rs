use std::{ops::Deref, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{header, Client, Method, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use crate::{ExtraArgs, ResponseProfile};

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
        let content_type = get_content_type(&headers);

        let body: Result<String> = match content_type.as_deref() {
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

        let client = Client::builder().build()?;
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

#[derive(Debug)]
pub struct ResponseExt(Response);

impl ResponseExt {
    pub async fn get_text(self, profile: &Option<ResponseProfile>) -> Result<String> {
        if let Some(profile) = profile {
            let mut output = String::new();

            output.push_str(&self.get_headers(&profile.skip_headers)?);

            // application/json; charset=utf-8
            let content_type = get_content_type(self.headers());
            let text = self.0.text().await?;

            match content_type.as_deref() {
                Some("application/json") => {
                    let text = filter_json(&text, &profile.skip_body)?;
                    output.push_str(&text);
                }
                _ => return Ok(text),
            };

            Ok(output)
        } else {
            Ok(self.0.text().await?)
        }
    }
    fn get_headers(&self, skip_headers: &[String]) -> Result<String> {
        let mut output = String::new();

        output.push_str(&format!("{:?} {}\r\n", self.version(), self.status()));

        for header in self.headers().iter() {
            if !skip_headers.contains(&header.0.to_string()) {
                output.push_str(&format!("{}: {}\n", header.0, header.1.to_str()?));
            }
        }
        output.push('\n');

        Ok(output)
    }
}

impl Deref for ResponseExt {
    type Target = Response;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn filter_json(text: &str, skip_body: &[String]) -> Result<String> {
    let mut json: serde_json::Value = serde_json::from_str(text)?;

    // TODO: support array of objects
    if let serde_json::Value::Object(ref mut obj) = json {
        for k in skip_body {
            obj.remove(k);
        }
    }

    Ok(serde_json::to_string_pretty(&json)?)
}
fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().unwrap().split(';').next().map(|v| v.to_string()))
}
