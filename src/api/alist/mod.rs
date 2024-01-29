use anyhow::Result;
use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;

use crate::common::log::LogResult;

mod auth;
mod fs;

pub struct Client {
    pub client: reqwest::Client,
    pub url: String,
    pub token: Option<String>,
}

impl Client {
    pub fn new(url: &str) -> Self {
        let client = reqwest::Client::new();
        let url = url.to_string();
        let token = None;
        Self { client, url, token }
    }

    fn auth(&self, request: RequestBuilder) -> Result<RequestBuilder> {
        Ok(request.header("Authorization", self.token()?))
    }

    fn token(&self) -> Result<&str> {
        if let Some(token) = self.token.as_deref() {
            return Ok(token);
        }
        anyhow::bail!("token not found");
    }
}

#[async_trait::async_trait]
trait JsonOrLog<T> {
    async fn json_or_log(self) -> Result<T>;
}

#[async_trait::async_trait]
impl<T> JsonOrLog<T> for Response
where
    T: DeserializeOwned,
{
    async fn json_or_log(self) -> Result<T> {
        let text = self.text().await.log()?;
        let response: T = match serde_json::from_str(text.as_str()) {
            Ok(response) => response,
            Err(e) => {
                tracing::error!("Response: {}", text);
                return Err(e).log();
            }
        };
        Ok(response)
    }
}
