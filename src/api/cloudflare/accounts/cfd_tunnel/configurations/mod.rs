use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::ClientCfdTunnel;

mod get;

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
