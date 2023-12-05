use anyhow::Result;
use serde::Deserialize;

use crate::api::cloudflare::auth::Auth;
use crate::api::cloudflare::dns::Record;
use crate::api::cloudflare::Client;
use crate::common::log::LogResult;

impl Client {
    pub async fn list(&self, zone: &str, name: &str) -> Result<Vec<Record>> {
        let response = self
            .client
            .get(&format!("{}/zones/{}/dns_records", self.api, zone))
            .auth(&self.token)
            .query(&[("name", name)])
            .send()
            .await
            .log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json().await.log()?;
        crate::ensure!(response.success);
        Ok(response.result)
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    result: Vec<Record>,
    success: bool,
}
