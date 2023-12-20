use anyhow::Result;
use reqwest::{IntoUrl, RequestBuilder, Url};

pub mod proxies;

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    url: Url,
    secret: Option<String>,
}

impl Client {
    pub fn new<U>(url: U, secret: Option<String>) -> Result<Self>
    where
        U: IntoUrl,
    {
        Ok(Self {
            client: reqwest::Client::new(),
            url: url.into_url()?,
            secret,
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
