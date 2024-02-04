use std::net::IpAddr;

use anyhow::Result;
use clap::Args;
use tabled::settings::{object::Columns, style::VerticalLine, Color, Concat, Style};

use claps::api::ipsb::IPVersion;

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    addr: Option<IpAddr>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        if let Some(addr) = self.addr {
            let geoip = claps::api::ipsb::geoip(Some(addr), None).await?;
            let mut table = geoip.table();
            table.with(Style::empty());
            table.modify(Columns::single(0), Color::FG_BLUE);
            table.modify(Columns::single(1), Color::FG_YELLOW);
            println!("{}", table);
        } else {
            let geoipv4 = claps::api::ipsb::geoip(None, Some(IPVersion::V4));
            let geoipv6 = claps::api::ipsb::geoip(None, Some(IPVersion::V6));
            let (geoipv4, geoipv6) = tokio::join!(geoipv4, geoipv6);
            let mut table = if let Ok(geoip) = geoipv4.as_ref() {
                let mut table = geoip.table();
                if let Ok(geoip) = geoipv6.as_ref() {
                    table.with(Concat::horizontal(geoip.table()));
                }
                table
            } else {
                geoipv6?.table()
            };
            table.modify(Columns::single(0), Color::FG_BLUE);
            table.modify(Columns::single(1), Color::FG_YELLOW);
            if table.count_columns() <= 2 {
                table.with(Style::empty());
            } else {
                table.with(Style::empty().verticals([(2, VerticalLine::new('|'))]));
                table.modify(Columns::single(2), Color::FG_BLUE);
                table.modify(Columns::single(3), Color::FG_YELLOW);
            }
            println!("{}", table);
        }
        Ok(())
    }
}
