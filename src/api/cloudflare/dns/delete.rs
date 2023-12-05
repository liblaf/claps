use anyhow::Result;

use crate::api::cloudflare::auth::Auth;
use crate::api::cloudflare::dns::Record;
use crate::api::cloudflare::Client;
use crate::common::log::LogResult;

impl Client {
    pub async fn delete(&self, zone: &str, record: &Record) -> Result<()> {
        let response = self
            .client
            .delete(format!(
                "{}/zones/{}/dns_records/{}",
                self.api, zone, record.id
            ))
            .auth(&self.token)
            .send()
            .await
            .log()?;
        response.error_for_status().log()?;
        tracing::info!(
            {
                name = record.name,
                content = record.content,
            },
            "Delete DNS Record"
        );
        Ok(())
    }
}
