use std::net::IpAddr;
use std::str::FromStr;

use anyhow::Result;
use clap::Args;

use claps::api::cloudflare::Client;

use super::CommonArgs;

#[derive(Debug, Args)]
pub(super) struct Cmd {}

impl Cmd {
    pub async fn run(self, args: CommonArgs) -> Result<()> {
        let name = args.name()?;
        let token = args.token()?;
        let zone = args.zone()?;
        let client = Client::new(token);
        let records = client.list(&zone, &name).await?;
        let mut local_addrs = crate::ip::addrs()?;
        for record in records {
            let remote_addr = IpAddr::from_str(&record.content)?;
            match local_addrs
                .iter()
                .position(|local_addr| *local_addr == remote_addr)
            {
                Some(index) => {
                    local_addrs.remove(index);
                }
                None => client.delete(&zone, &record).await?,
            }
        }
        for local_addr in local_addrs {
            client.create(&zone, &name, local_addr).await?;
        }
        Ok(())
    }
}
