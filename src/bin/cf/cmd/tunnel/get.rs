use anyhow::Result;
use clap::Args;

use tabled::{
    builder::Builder,
    settings::{
        location::Locator,
        object::{Cell, Columns, Object, Rows},
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
    account: String,
    #[arg(from_global)]
    zone: String,
    #[arg(from_global)]
    name: Option<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let client = crate::helper::client::accounts(
            self.api.as_str(),
            self.token.as_deref(),
            self.account.as_str(),
        )
        .await?
        .cfd_tunnel();
        let tunnel = client
            .get(Some(self.name.unwrap_or_else(whoami::devicename).as_str()))
            .await?;
        let tunnel = tunnel.first().unwrap();
        let client = client.configurations(tunnel.id.as_str());
        let config = client.get().await?;
        let client_dns = crate::helper::client::zones(
            self.api.as_str(),
            self.token.as_deref(),
            self.zone.as_str(),
        )
        .await?
        .dns_records();
        let records = client_dns.get(None).await?;
        let mut table = Builder::new();
        table.push_record(["Hostname", "", "Service", "DNS Status"]);
        for ingress in config.ingress {
            let status = if let Some(hostname) = ingress.hostname.as_deref() {
                match records.iter().find(|r| {
                    r.name == hostname
                        && r.content == crate::helper::domain::tunnel(tunnel.id.as_str())
                }) {
                    Some(_) => "Healthy",
                    None => "Down",
                }
            } else {
                ""
            };
            table.push_record([
                ingress.hostname.as_deref().unwrap_or_default(),
                "->",
                ingress.service.as_str(),
                status,
            ]);
        }
        let mut table = table.build();
        table
            .with(Style::empty())
            .modify(Columns::first(), Alignment::right())
            .modify(Cell::new(0, 0), Color::BOLD | Color::FG_BLUE)
            .modify(Cell::new(0, 2), Color::BOLD | Color::FG_YELLOW)
            .modify(Cell::new(0, 3), Color::BOLD)
            .modify(Columns::first().not(Rows::first()), Color::FG_BLUE)
            .modify(Columns::single(2).not(Rows::first()), Color::FG_YELLOW)
            .modify(Locator::content("Healthy"), Color::FG_GREEN)
            .modify(Locator::content("Down"), Color::FG_RED);
        println!("{}", table);
        println!("Tunnel Status: {}", tunnel.status);
        Ok(())
    }
}
