use anyhow::Result;
use clap::Args;
use tabled::{
    builder::Builder,
    settings::{
        object::{Cell, Columns},
        Alignment, Color, Style,
    },
};

use super::AccountsArgsFromGlobal;

#[derive(Args)]
pub struct Cmd {
    #[command(flatten)]
    args: AccountsArgsFromGlobal,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let client = self.args.accounts().await?;
        let client = client.cfd_tunnel();
        let cfd_tunnel = client.get(Some(whoami::devicename().as_str())).await?;
        let cfd_tunnel = cfd_tunnel.first().unwrap();
        let client = client.configurations(cfd_tunnel.id.as_str());
        let config = client.get().await?;
        let mut table = Builder::new();
        table.push_record(["Hostname", "", "Service"]);
        for ingress in config.ingress {
            table.push_record([
                ingress.hostname.unwrap_or_default().as_str(),
                "->",
                ingress.service.as_str(),
            ]);
        }
        let mut table = table.build();
        table
            .with(Style::empty())
            .modify(Columns::first(), Alignment::right())
            .modify(Columns::first(), Color::FG_BLUE)
            .modify(Columns::last(), Color::FG_YELLOW)
            .modify(Cell::new(0, 0), Color::BOLD | Color::FG_BLUE)
            .modify(Cell::new(0, 2), Color::BOLD | Color::FG_YELLOW);
        println!("{}", table);
        Ok(())
    }
}
