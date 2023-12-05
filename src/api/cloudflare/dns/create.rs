use std::net::IpAddr;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::cloudflare::auth::Auth;
use crate::api::cloudflare::Client;
use crate::common::log::LogResult;

impl Client {
    pub async fn create(&self, zone: &str, name: &str, content: IpAddr) -> Result<()> {
        let request = Request {
            content: content.to_string(),
            name: name.to_string(),
            proxied: false,
            ttl: 60,
            type_: match content {
                IpAddr::V4(_) => "A",
                IpAddr::V6(_) => "AAAA",
            }
            .to_string(),
        };
        let response = self
            .client
            .post(&format!("{}/zones/{}/dns_records", self.api, zone))
            .auth(&self.token)
            .json(&request)
            .send()
            .await
            .log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json().await.log()?;
        anyhow::ensure!(response.success);
        tracing::info!(
            {
                name = name,
                content = content.to_string(),
            },
            "Create DNS Record"
        );
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct Request {
    content: String,
    name: String,
    proxied: bool,
    #[serde(rename = "type")]
    type_: String,
    ttl: i32,
}

#[derive(Debug, Deserialize)]
struct Response {
    success: bool,
}
