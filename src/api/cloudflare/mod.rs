use reqwest::{header::HeaderMap, ClientBuilder};
use serde::{Deserialize, Serialize};

use crate::common::log::LogResult;

pub mod zones;

pub const API: &str = "https://api.cloudflare.com/client/v4";

pub struct Cloudflare {
    client: reqwest::Client,
    api: String,
    token: String,
}

impl Cloudflare {
    pub fn new(api: Option<&str>, token: &str) -> Self {
        Self {
            api: api.unwrap_or(API).to_string(),
            client: ClientBuilder::new()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {}", token).parse().log().unwrap(),
                    );
                    headers
                })
                .build()
                .log()
                .unwrap(),
            token: token.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    code: i64,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultInfo {
    count: i64,
    page: i64,
    per_page: i64,
    total_count: i64,
}
