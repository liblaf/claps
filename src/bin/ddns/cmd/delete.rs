use anyhow::Result;
use clap::Args;

use claps::api::cloudflare::Client;
use claps::common::cmd::Run;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(flatten)]
    args: super::CommonArgs,
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let name = self.args.name()?;
        let token = self.args.token()?;
        let zone = self.args.zone()?;
        let client = Client::new(token);
        let records = client.list(&zone, &name).await?;
        for record in records {
            client.delete(&zone, &record).await?;
        }
        Ok(())
    }
}
