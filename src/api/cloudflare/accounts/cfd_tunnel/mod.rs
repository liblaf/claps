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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Inactive,
    Degraded,
    Healthy,
    Down,
}
