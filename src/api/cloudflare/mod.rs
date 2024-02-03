use serde::Deserialize;

mod create;
mod delete;
mod list;
mod update;

const API: &str = "https://api.cloudflare.com/client/v4";

pub struct Client {
    api: String,
    client: reqwest::Client,
    token: String,
    zone: String,
}

impl Client {
    pub fn new(token: &str, zone: &str) -> Self {
        Self {
            api: API.to_string(),
            client: reqwest::Client::new(),
            token: token.to_string(),
            zone: zone.to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct DNSRecord {
    pub content: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
}
