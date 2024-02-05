use anyhow::Result;
use clap::Args;
use tabled::{
    builder::Builder,
    settings::{object::Columns, Alignment, Color, Style},
};

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
        let records = zones.dns_records().get(Some(name.as_str())).await?;
        let mut table = Builder::new();
        table.push_record(["Name", "Type", "TTL", "ADDRESS"]);
        for record in records {
            table.push_record([
                record.name.as_str(),
                record.type_.as_str(),
                format!("{}s", record.ttl).as_str(),
                record.content.as_str(),
            ]);
        }
        let mut table = table.build();
        table
            .with(Style::empty())
            .modify(Columns::first(), Color::BOLD | Color::FG_GREEN)
            .modify(Columns::single(1), Color::BOLD | Color::FG_BLUE)
            .modify(Columns::single(2), Alignment::right());
        println!("{}", table);
        Ok(())
    }
}
