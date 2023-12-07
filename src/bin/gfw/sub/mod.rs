use anyhow::Result;
use chrono::{DateTime, Utc};

use claps::common::log::LogResult;
use claps::external::bitwarden::Item;

#[derive(Debug, Default)]
pub struct UserInfo {
    pub download: u64,
    pub upload: u64,
    pub total: u64,
    pub expire: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Sub {
    pub name: String,
    pub url: Option<String>,
    pub user_info: Option<UserInfo>,
}

impl Sub {
    pub async fn from(item: &Item) -> Result<Self> {
        Ok(Sub {
            name: item.name.to_owned(),
            url: item.notes.to_owned(),
            user_info: match item.notes.as_deref() {
                Some(url) => user_info(url).await.log().ok(),
                None => None,
            },
        })
    }
}

async fn user_info(url: &str) -> Result<UserInfo> {
    let response = reqwest::get(url).await.log()?;
    let response = response.error_for_status().log()?;
    let header = response
        .headers()
        .get("subscription-userinfo")
        .unwrap()
        .to_str()
        .log()?;
    let mut user_info = UserInfo::default();
    for segment in header.split(';') {
        let segment = segment.trim();
        if let Some((key, value)) = segment.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "download" => user_info.download = value.parse().log().unwrap_or_default(),
                "upload" => user_info.upload = value.parse().log().unwrap_or_default(),
                "total" => user_info.total = value.parse().log().unwrap_or_default(),
                "expire" => {
                    user_info.expire =
                        DateTime::from_timestamp(value.parse().log().unwrap_or_default(), 0)
                            .unwrap_or_default()
                }
                _ => tracing::error!("invalid header: {}", segment),
            }
        } else {
            tracing::error!("invalid header: {}", segment)
        }
    }
    Ok(user_info)
}
