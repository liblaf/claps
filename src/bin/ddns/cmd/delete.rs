use anyhow::Result;
use clap::Args;

use claps::api::cloudflare::Client;

use crate::cmd::GlobalArgs;

#[derive(Args)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, args: GlobalArgs) -> Result<()> {
        let client = Client::new(args.token().await?.as_str(), args.zone.as_str());
        let records = client.list(args.name()?.as_str()).await?;
        let jobs = records.iter().map(|r| client.delete(r)).collect::<Vec<_>>();
        futures::future::join_all(jobs)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;
        Ok(())
    }
}
