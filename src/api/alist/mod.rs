use anyhow::Result;

mod auth;
mod fs;

pub struct Client {
    pub client: reqwest::Client,
    pub url: String,
    pub token: Option<String>,
}

impl Client {
    pub fn new(url: &str) -> Self {
        let client = reqwest::Client::new();
        let url = url.to_string();
        let token = None;
        Self { client, url, token }
    }

    fn token(&self) -> Result<&str> {
        if let Some(token) = self.token.as_deref() {
            return Ok(token);
        }
        anyhow::bail!("token not found");
    }
}
