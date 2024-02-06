use std::fmt::{Display, Formatter};

use colored::Colorize;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::ClientAccounts;

pub mod configurations;
mod get;

pub struct ClientCfdTunnel {
    client: Client,
    api: String,
    token: String,
    account_id: String,
}

impl ClientAccounts {
    pub fn cfd_tunnel(&self) -> ClientCfdTunnel {
        ClientCfdTunnel {
            client: self.client.to_owned(),
            api: self.api.to_string(),
            token: self.token.to_string(),
            account_id: self.account_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfdTunnel {
    pub id: String,
    pub name: String,
    pub connections: Vec<Connection>,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub is_pending_reconnect: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Healthy,
    Degraded,
    Down,
    Inactive,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Status::Inactive => write!(f, "{}", "Inactive".dimmed()),
            Status::Degraded => write!(f, "{}", "Degraded".yellow()),
            Status::Healthy => write!(f, "{}", "Healthy".green()),
            Status::Down => write!(f, "{}", "Down".red()),
        }
    }
}
