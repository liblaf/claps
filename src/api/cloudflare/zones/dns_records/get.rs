use anyhow::Result;

use crate::{api::cloudflare::zones::dns_records::DnsRecord, common::log::LogResult};

use super::ClientDnsRecords;

impl ClientDnsRecords {
    pub async fn get(&self, name: Option<&str>) -> Result<Vec<DnsRecord>> {
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
        let result = crate::api::cloudflare::handle::<Vec<DnsRecord>>(response).await?;
        Ok(result)
    }
}
