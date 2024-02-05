use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Accounts;

pub mod configurations;
mod get;

pub struct ClientCfdTunnel {
    client: Client,
    api: String,
    token: String,
    account_id: String,
}

impl Accounts {
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
}
