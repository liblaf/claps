use anyhow::Result;
use clap::Args;

use colored::Colorize;
use tabled::builder::Builder;
use tabled::settings::Style;

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
        let mut builder = Builder::new();
        builder.push_record(["Name", "Address"]);
        records.iter().for_each(|record| {
            builder.push_record([
                record.name.bold().green().to_string(),
                record.content.bold().blue().to_string(),
            ]);
        });
        let mut table = builder.build();
        table.with(Style::rounded());
        println!("{}", table);
        Ok(())
    }
}
