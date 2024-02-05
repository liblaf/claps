use std::net::IpAddr;

use anyhow::Result;
use clap::Args;

use super::ZonesArgsFromGlobal;

#[derive(Args)]
pub struct Cmd {
    #[command(flatten)]
    args: ZonesArgsFromGlobal,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let mut addrs: Vec<IpAddr> = claps::external::py::ip().await?;
        let name = self.args.name().await?;
        let zones = self.args.zones().await?;
        let dns_records = zones.dns_records();
        let records = dns_records.get(Some(name.as_str())).await?;
        let mut jobs_delete = vec![];
        for record in records.as_slice() {
            let addr = record.content.parse::<IpAddr>()?;
            let pos = addrs.iter().position(|a| a == &addr);
            if let Some(pos) = pos {
                addrs.remove(pos);
            } else {
                jobs_delete.push(dns_records.delete(record.id.as_str(), Some(record)));
            }
        }
        let mut jobs_create = vec![];
        for addr in addrs.as_slice() {
            jobs_create.push(
                dns_records.post(
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
        let (results_delete, results_create) = futures::future::join(
            futures::future::join_all(jobs_delete),
            futures::future::join_all(jobs_create),
        )
        .await;
        results_delete.into_iter().collect::<Result<Vec<_>>>()?;
        results_create.into_iter().collect::<Result<Vec<_>>>()?;
        Ok(())
    }
}
