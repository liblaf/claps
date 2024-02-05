use reqwest::Client;

use super::Cloudflare;

pub mod cfd_tunnel;

pub struct ClientAccounts {
    client: Client,
    api: String,
    token: String,
    account_id: String,
}

impl Cloudflare {
    pub fn accounts(&self, account_id: &str) -> ClientAccounts {
        ClientAccounts {
            client: self.client.to_owned(),
            api: self.api.to_string(),
            token: self.token.to_string(),
            account_id: account_id.to_string(),
        }
    }
}
