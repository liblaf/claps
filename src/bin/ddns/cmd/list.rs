use anyhow::Result;
use clap::Args;

use colored::Colorize;
use tabled::builder::Builder;
use tabled::settings::Style;

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
