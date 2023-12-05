mod auth;
pub mod dns;

pub struct Client {
    api: String,
    client: reqwest::Client,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            api: API.to_string(),
            client: reqwest::Client::new(),
            token,
        }
    }
}

const API: &str = "https://api.cloudflare.com/client/v4";
