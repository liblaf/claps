use std::net::IpAddr;

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Cmd {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    zone: String,
    #[arg(from_global)]
    name: Option<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let mut addrs: Vec<IpAddr> = claps::external::py::ip().await?;
        let name = format!(
            "{}.ddns.liblaf.me",
            self.name.unwrap_or_else(whoami::hostname)
        );
        let client = crate::helper::client::zones(
            self.api.as_str(),
            self.token.as_deref(),
            self.zone.as_str(),
        )
        .await?;
        let client = client.dns_records();
        let records = client.get(Some(name.as_str())).await?;
        let mut jobs_delete = vec![];
        for record in records.as_slice() {
            let addr = record.content.parse::<IpAddr>()?;
            let pos = addrs.iter().position(|a| a == &addr);
            if let Some(pos) = pos {
                tracing::info!("DNS Record Exists: {}", record);
                addrs.remove(pos);
            } else {
                jobs_delete.push(client.delete(record.id.as_str(), Some(record)));
            }
        }
        let mut jobs_create = vec![];
        for addr in addrs.as_slice() {
            jobs_create.push(
                client.post(
                    addr.to_string(),
                    name.to_string(),
                    Some(false),
                    match addr {
                        IpAddr::V4(_) => "A",
                        IpAddr::V6(_) => "AAAA",
                    }
                    .to_string(),
                    Some(60),
                ),
            );
        }
        futures::future::try_join(
            futures::future::try_join_all(jobs_delete),
            futures::future::try_join_all(jobs_create),
        )
        .await?;
        Ok(())
    }
}
