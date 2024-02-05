use std::fmt::{Display, Formatter};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tabled::{
    builder::Builder,
    settings::{
        object::{Cell, Columns},
        Alignment, Color, Style,
    },
};

use super::ClientCfdTunnel;

mod get;
mod put;

pub struct ClientConfigurations {
    client: Client,
    api: String,
    token: String,
    account_id: String,
    tunnel_id: String,
}

impl ClientCfdTunnel {
    pub fn configurations(&self, tunnel_id: &str) -> ClientConfigurations {
        ClientConfigurations {
            client: self.client.to_owned(),
            api: self.api.to_string(),
            token: self.token.to_string(),
            account_id: self.account_id.to_string(),
            tunnel_id: tunnel_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub ingress: Vec<Ingress>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingress {
    pub hostname: Option<String>,
    pub service: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut table = Builder::new();
        table.push_record(["Hostname", "", "Service"]);
        for ingress in self.ingress.as_slice() {
            table.push_record([
                ingress.hostname.as_deref().unwrap_or_default(),
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
        write!(f, "{}", table)
    }
}
