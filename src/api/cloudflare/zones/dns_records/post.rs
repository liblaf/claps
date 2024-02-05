use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    api::cloudflare::ResponseObject,
    common::log::{LogJson, LogResult},
};

use super::{DNSRecord, DNSRecords};

impl DNSRecords {
    pub async fn post(
        &self,
        content: String,
        name: String,
        proxied: Option<bool>,
        type_: String,
        ttl: Option<u64>,
    ) -> Result<()> {
        let request = self
            .client
            .post(format!("{}/zones/{}/dns_records", self.api, self.zone_id))
            .json(&Request {
                content,
                name,
                proxied,
                type_,
                ttl,
            })
            .bearer_auth(self.token.as_str());
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response = response
            .json_log::<ResponseObject<DNSRecord>>()
            .await?
            .log();
        tracing::info!("Create DNS Record: {}", response.result);
        Ok(())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Request {
    content: String,
    name: String,
    proxied: Option<bool>,
    #[serde(rename = "type")]
    type_: String,
    ttl: Option<u64>,
}
