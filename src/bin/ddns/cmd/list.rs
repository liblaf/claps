use anyhow::Result;
use clap::Args;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Color, Style};

use claps::api::cloudflare::Client;

use crate::cmd::GlobalArgs;

#[derive(Args)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self, args: GlobalArgs) -> Result<()> {
        let mut table = Builder::new();
        table.push_record(["Name", "Type", "Content"]);
        let client = Client::new(args.token().await?.as_str(), args.zone.as_str());
        let records = client.list(args.name()?.as_str()).await?;
        for record in records {
            table.push_record([record.name, record.type_, record.content]);
        }
        let mut table = table.build();
        table
            .with(Style::rounded())
            .modify(Columns::single(0), Color::BOLD | Color::FG_GREEN)
            .modify(Columns::single(1), Color::BOLD | Color::FG_BLUE);
        println!("{}", table);
        Ok(())
    }
}
