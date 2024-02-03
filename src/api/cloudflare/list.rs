use anyhow::Result;
use serde::Deserialize;

use crate::api::cloudflare::{Client, DNSRecord};
use crate::common::log::LogResult;

impl Client {
    pub async fn list(&self, name: &str) -> Result<Vec<DNSRecord>> {
        let request = self
            .client
            .get(format!("{}/zones/{}/dns_records", self.api, self.zone))
            .bearer_auth(self.token.as_str())
            .query(&serde_json::json!({ "name": name }));
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json().await.log()?;
        crate::ensure!(response.success);
        Ok(response.result)
    }
}

#[derive(Deserialize)]
struct Response {
    result: Vec<DNSRecord>,
    success: bool,
}
