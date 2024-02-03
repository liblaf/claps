mod auth;
mod fs;

pub struct Client {
    client: reqwest::Client,
    token: Option<String>,
    url: String,
}

impl Client {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: reqwest::Client::new(),
            token: None,
        }
    }
}
