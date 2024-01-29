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
        for record in records {
            client.delete(&zone, &record).await?;
        }
        Ok(())
    }
}
