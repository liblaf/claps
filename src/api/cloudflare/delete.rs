use anyhow::Result;
use serde::Deserialize;

use crate::api::cloudflare::{Client, DNSRecord};
use crate::common::log::LogResult;

impl Client {
    pub async fn delete(&self, record: &DNSRecord) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{}/zones/{}/dns_records/{}",
                self.api, self.zone, record.id
            ))
            .bearer_auth(self.token.as_str());
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json().await.log()?;
        crate::ensure!(response.result.id == record.id);
        tracing::info!({ name = record.name, addr = record.content }, "Delete DNS Record");
        Ok(())
    }
}

#[derive(Deserialize)]
struct Response {
    result: ResponseResult,
}

#[derive(Deserialize)]
struct ResponseResult {
    id: String,
}
