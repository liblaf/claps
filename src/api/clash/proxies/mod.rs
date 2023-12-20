use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::api::clash::Client;
use crate::common::log::LogResult;

impl Client {
    pub async fn proxies(&self) -> Result<HashMap<String, Proxy>> {
        let url = self.url.join("/proxies")?;
        let response = self.get(url)?.send().await.log()?;
        response.error_for_status_ref().log()?;
        let response: Response = response.json().await?;
        Ok(response.proxies)
    }

    pub async fn proxy_set(&self, proxy: &str, name: &str) -> Result<()> {
        let url = self.url.join(format!("/proxies/{}", proxy).as_str())?;
        self.put(url)?
            .json(&serde_json::json!({ "name": name }))
            .send()
            .await
            .log()?
            .error_for_status_ref()
            .log()?;
        tracing::info!("{} -> {}", proxy, name);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    proxies: HashMap<String, Proxy>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Proxy {
    pub all: Option<Vec<String>>,
    pub history: Vec<History>,
    pub name: String,
    pub now: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct History {
    pub delay: u64,
    pub time: String,
}
