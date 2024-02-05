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
        let name = self.args.name().await?;
        let zones = self.args.zones().await?;
        let dns_records = zones.dns_records();
        let records = dns_records.get(Some(name.as_str())).await?;
        let tasks = records
            .iter()
            .map(|record| dns_records.delete(record.id.as_str(), Some(record)))
            .collect::<Vec<_>>();
        futures::future::try_join_all(tasks).await?;
        Ok(())
    }
}
