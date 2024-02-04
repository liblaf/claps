use std::net::IpAddr;

use anyhow::Result;
use clap::Args;

use claps::api::cloudflare::Client;

use crate::cmd::GlobalArgs;

#[derive(Args)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, args: GlobalArgs) -> Result<()> {
        let mut addrs = claps::external::py::ip().await?;
        let client = Client::new(args.token().await?.as_str(), args.zone.as_str());
        let records = client.list(args.name()?.as_str()).await?;
        let mut jobs_delete = vec![];
        for record in records.as_slice() {
            let addr = record.content.parse::<IpAddr>()?;
            let pos = addrs.iter().position(|a| a == &addr);
            if let Some(pos) = pos {
                addrs.remove(pos);
            } else {
                jobs_delete.push(client.delete(record));
            }
        }
        let name = args.name()?;
        let mut jobs_create = vec![];
        for addr in addrs.as_slice() {
            jobs_create.push(client.create(name.as_str(), addr));
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
