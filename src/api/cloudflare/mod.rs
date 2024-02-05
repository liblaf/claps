use anyhow::Result;
use reqwest::{header::HeaderMap, ClientBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::common::log::{LogError, LogJson, LogResult};

pub mod accounts;
pub mod zones;

pub const API: &str = "https://api.cloudflare.com/client/v4";

pub struct Cloudflare {
    client: reqwest::Client,
    api: String,
    token: String,
}

impl Cloudflare {
    pub fn new(api: Option<&str>, token: &str) -> Self {
        Self {
            api: api.unwrap_or(API).to_string(),
            client: ClientBuilder::new()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {}", token).parse().log().unwrap(),
                    );
                    headers
                })
                .build()
                .log()
                .unwrap(),
            token: token.to_string(),
        }
    }
}

async fn handle<T>(response: reqwest::Response) -> Result<T>
where
    T: DeserializeOwned,
{
    match response.error_for_status_ref() {
        Ok(_) => {
            let response = response.json_log::<Response<T>>().await?;
            crate::ensure!(response.success);
            Ok(response.result)
        }
        Err(err) => {
            let err = err.log();
            let response = response.json_log::<Response<Value>>().await?;
            response.log();
            Err(err)
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    errors: Vec<Message>,
    messages: Vec<Message>,
    result: T,
    success: bool,
    result_info: Option<ResultInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    code: i64,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultInfo {
    count: i64,
    page: i64,
    per_page: i64,
    total_count: i64,
}

impl<T> Response<T> {
    fn log(self) -> Self {
        for error in self.errors.as_slice() {
            tracing::error!(code = error.code, message = error.message)
        }
        for message in self.messages.as_slice() {
            tracing::info!(code = message.code, message = message.message)
        }
        self
    }
}
