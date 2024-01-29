use anyhow::Result;
use clap::Args;

use claps::api::clash::Client;

use super::CommonArgs;

#[derive(Debug, Args)]
pub(super) struct Cmd {}

impl Cmd {
    pub async fn run(&self, args: CommonArgs) -> Result<()> {
        let client = Client::new(args.url, args.secret)?;
        let pretties = crate::cmd::proxy::pretty(&client, "PROXY").await?;
        for pretty in pretties {
            println!("{}", pretty);
        }
        Ok(())
    }
}
