use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::{Client, IntoUrl};

use crate::common::log::LogResult;
use crate::internal::sub::Sub;

#[derive(Debug)]
pub struct UserInfo {
    pub upload: u64,
    pub download: u64,
    pub total: u64,
    pub expire: DateTime<Utc>,
}

async fn get_info<U>(url: U) -> Result<UserInfo>
where
    U: IntoUrl,
{
    let client = Client::new();
    let request = client.get(url);
    let response = request.send().await.log()?;
    let header = response
        .headers()
        .get("Subscription-Userinfo")
        .ok_or(anyhow::anyhow!("Subscription-Userinfo Not Found"))
        .log()?
        .to_str()
        .log()?;
    let mut info = UserInfo {
        upload: 0,
        download: 0,
        total: 0,
        expire: DateTime::default(),
    };
    for split in header.split(';') {
        let (key, value) = split
            .split_once('=')
            .ok_or(anyhow::anyhow!("Invalid Subscription-Userinfo"))?;
        let key = key.trim();
        let value = value.trim();
        match key {
            "upload" => info.upload = value.parse().log()?,
            "download" => info.download = value.parse().log()?,
            "total" => info.total = value.parse().log()?,
            "expire" => {
                info.expire = DateTime::from_timestamp(value.parse().log()?, 0)
                    .ok_or(anyhow::anyhow!("Invalid Timestamp"))?
            }
            _ => {}
        }
    }
    Ok(info)
}

impl Sub {
    pub async fn with_info(self) -> Self {
        if let Some(url) = self.url.as_ref() {
            Self {
                user_info: get_info(url.as_ref()).await.log().ok(),
                ..self
            }
        } else {
            self
        }
    }
}
