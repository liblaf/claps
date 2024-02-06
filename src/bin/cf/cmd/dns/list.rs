use anyhow::Result;
use clap::Args;
use tabled::{
    builder::Builder,
    settings::{
        object::{Cell, Columns, Rows},
        Alignment, Color, Style,
    },
};

#[derive(Args)]
pub struct Cmd {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    zone: String,
    #[arg(from_global)]
    name: Option<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let name = self.name.unwrap_or_else(whoami::hostname);
        let client = crate::helper::client::zones(
            self.api.as_str(),
            self.token.as_deref(),
            self.zone.as_str(),
        )
        .await?;
        let client = client.dns_records();
        let records = client.get(None).await?;
        let records = records
            .into_iter()
            .filter(|r| r.name.contains(name.as_str()))
            .collect::<Vec<_>>();
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
            .modify(Columns::single(2), Alignment::right())
            .modify(Columns::first(), Color::FG_GREEN)
            .modify(Columns::single(1), Color::FG_BLUE)
            .modify(Rows::first(), Color::BOLD)
            .modify(Cell::new(0, 0), Color::BOLD | Color::FG_GREEN)
            .modify(Cell::new(0, 1), Color::BOLD | Color::FG_BLUE);
        println!("{}", table);
        Ok(())
    }
}
