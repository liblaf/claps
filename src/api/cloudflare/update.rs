use std::net::IpAddr;

use anyhow::Result;
use serde::Deserialize;

use crate::api::cloudflare::{Client, DNSRecord};
use crate::common::log::LogResult;

impl Client {
    pub async fn put(&self, id: &str, name: &str, addr: &IpAddr) -> Result<DNSRecord> {
        let request = self
            .client
            .put(format!(
                "{}/zones/{}/dns_records/{}",
                self.api, self.zone, id
            ))
            .bearer_auth(self.token.as_str())
            .json(&serde_json::json!({
                "content": addr,
                "name": name,
                "proxied": false,
                "type": match addr {
                    IpAddr::V4(_) => "A",
                    IpAddr::V6(_) => "AAAA",
                },
                "ttl": 60,
            }));
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json().await.log()?;
        crate::ensure!(response.success);
        tracing::info!({ name = name, addr = ?addr }, "Update DNS Record");
        Ok(response.result)
    }
}

#[derive(Deserialize)]
struct Response {
    result: DNSRecord,
    success: bool,
}
