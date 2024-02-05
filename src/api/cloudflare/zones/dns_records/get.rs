use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    api::cloudflare::{
        zones::dns_records::{DNSRecord, DNSRecords},
        Message, ResultInfo,
    },
    common::log::{LogJson, LogResult},
};

impl DNSRecords {
    pub async fn get(&self, name: Option<&str>) -> Result<Vec<DNSRecord>> {
        let request = self
            .client
            .get(format!("{}/zones/{}/dns_records", self.api, self.zone_id))
            .bearer_auth(self.token.as_str());
        let request = if let Some(name) = name {
            request.query(&[("name", name)])
        } else {
            request
        };
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response = response.json_log::<Response>().await?.log();
        crate::ensure!(response.success);
        Ok(response.result)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    result: Vec<DNSRecord>,
    errors: Vec<Message>,
    messages: Vec<Message>,
    success: bool,
    result_info: ResultInfo,
}

impl Response {
    fn log(self) -> Self {
        for error in self.errors.as_slice() {
            tracing::error!(code = error.code, message = error.message)
        }
        for message in self.messages.as_slice() {
            tracing::info!(code = message.code, message = message.message)
        }
        self
    }
}
