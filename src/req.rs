use std::ops::Deref;

use anyhow::{Ok, Result};
use reqwest::header::HeaderMap;
use reqwest::{header, Client, Response};

use crate::config::{ExtraArgs, Profile, RequestProfile, ResponseProfile};
use crate::utils::diff_text;

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

pub async fn send(rp: &RequestProfile, args: &ExtraArgs) -> Result<ResponseExt> {
    // 先合并 requestProfile 和 ExtraArgs
    let (headers, query, body) = rp.merge(args)?;

    let client = Client::builder().build()?;
    let req = client
        .request(rp.method.clone(), rp.url.clone())
        .query(&query)
        .headers(headers)
        .body(body)
        .build()?;

    let res = client.execute(req).await?;
    Ok(ResponseExt(res))
}

pub async fn diff(dp: &Profile, args: ExtraArgs) -> Result<String> {
    let res1 = send(&dp.req1, &args).await?;
    let res2 = send(&dp.req2, &args).await?;

    let text1 = res1.get_text(&dp.res).await?;
    let text2 = res2.get_text(&dp.res).await?;

    diff_text(&text1, &text2)
}
