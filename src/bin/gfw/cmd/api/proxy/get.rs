use anyhow::Result;
use clap::Args;

use claps::api::clash::Client;
use claps::common::cmd::Run;

use crate::cmd::api::CommandArgs;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(flatten)]
    common: CommandArgs,
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let client = Client::new(self.common.url, self.common.secret)?;
        let pretties = crate::cmd::api::proxy::pretty(&client, "PROXY").await?;
        for pretty in pretties {
            println!("{}", pretty);
        }
        Ok(())
    }
}
