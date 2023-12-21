use std::collections::HashMap;

use anyhow::Result;

use crate::api::clash::Client;
use crate::common::log::LogResult;

impl Client {
    pub async fn group_delay(&self, group: &str) -> Result<HashMap<String, u64>> {
        let url = self
            .url
            .join(format!("/group/{}/delay", group).as_str())
            .log()?;
        let response = self
            .get(url)?
            .query(&[("timeout", "2000"), ("url", "https://cp.cloudflare.com")])
            .send()
            .await
            .log()?
            .json()
            .await
            .log()?;
        Ok(response)
    }
}
