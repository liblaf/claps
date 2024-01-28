use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::api::clash::Client;
use crate::common::log::LogResult;

impl Client {
    pub async fn proxies(&self) -> Result<HashMap<String, Proxy>> {
        let url = self.url.join("/proxies").log()?;
        let response: Response = self
            .get(url)?
            .send()
            .await
            .log()?
            .error_for_status()
            .log()?
            .json()
            .await
            .log()?;
        Ok(response.proxies)
    }

    pub async fn proxy_set(&self, proxy: &str, name: &str) -> Result<()> {
        let url = self
            .url
            .join(format!("/proxies/{}", proxy).as_str())
            .log()?;
        self.put(url)?
            .json(&serde_json::json!({ "name": name }))
            .send()
            .await
            .log()?
            .error_for_status()
            .log()?;
        tracing::info!("{} -> {}", proxy, name);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    proxies: HashMap<String, Proxy>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub all: Option<Vec<String>>,
    pub history: Vec<History>,
    pub name: String,
    pub now: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize)]
pub struct History {
    pub delay: u64,
    pub time: String,
}
