use std::time::Duration;

use anyhow::Result;
use reqwest::{IntoUrl, RequestBuilder, Url};

pub mod group;
pub mod proxies;

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    url: Url,
    secret: Option<String>,
}

impl Client {
    pub fn new<U, S>(url: U, secret: Option<S>) -> Result<Self>
    where
        U: IntoUrl,
        S: Into<String>,
    {
        Ok(Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()?,
            url: url.into_url()?,
            secret: secret.map(|secret| secret.into()),
        })
    }

    fn get<U>(&self, url: U) -> Result<RequestBuilder>
    where
        U: IntoUrl,
    {
        let request = self.client.get(url);
        let request = if let Some(secret) = self.secret.as_deref() {
            request.bearer_auth(secret)
        } else {
            request
        };
        Ok(request)
    }

    fn put<U>(&self, url: U) -> Result<RequestBuilder>
    where
        U: IntoUrl,
    {
        let request = self.client.put(url);
        let request = if let Some(secret) = self.secret.as_deref() {
            request.bearer_auth(secret)
        } else {
            request
        };
        Ok(request)
    }
}
