use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::common::log::{LogJson, LogResult};

use super::{ClientDnsRecords, DnsRecord};

impl ClientDnsRecords {
    pub async fn delete(&self, dns_record_id: &str, dns_record: Option<&DnsRecord>) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{}/zones/{}/dns_records/{}",
                self.api, self.zone_id, dns_record_id
            ))
            .bearer_auth(self.token.as_str());
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response = response.json_log::<Response>().await?;
        if let Some(dns_record) = dns_record {
            tracing::info!("Delete Dns Record: {}", dns_record)
        } else {
            tracing::info!("Delete Dns Record: {}", response.result.id)
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    result: ResponseResult,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResponseResult {
    id: String,
}
