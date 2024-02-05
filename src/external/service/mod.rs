use std::fmt::{Display, Formatter};

use anyhow::Result;
use reqwest::Url;
use tokio::net::TcpListener;

use crate::{
    api::cloudflare::accounts::cfd_tunnel::configurations::Ingress,
    common::log::{LogError, LogResult},
};

pub enum Service {
    AList,
    BT,
    DNS,
    Glances,
    GPT,
    Jellyfin,
    PDF,
}

impl Display for Service {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::AList => write!(f, "alist"),
            Self::BT => write!(f, "bt"),
            Self::DNS => write!(f, "dns"),
            Self::Glances => write!(f, "glances"),
            Self::GPT => write!(f, "gpt"),
            Self::Jellyfin => write!(f, "jellyfin"),
            Self::PDF => write!(f, "pdf"),
        }
    }
}

impl From<&Service> for Ingress {
    fn from(val: &Service) -> Self {
        Ingress {
            hostname: Some(val.hostname()),
            service: val.service(),
        }
    }
}

impl Service {
    pub fn hostname(&self) -> String {
        format!("{}-{}.liblaf.me", self, whoami::hostname())
    }

    pub fn service(&self) -> String {
        match self {
            Self::AList => "http://127.0.0.1:5244".to_string(),
            Self::BT => "http://127.0.0.1:8000".to_string(),
            Self::DNS => "tcp://127.0.0.1:60053".to_string(),
            Self::Glances => "http://127.0.0.1:61208".to_string(),
            Self::GPT => "http://127.0.0.1:12303".to_string(),
            Self::Jellyfin => "http://127.0.0.1:8096".to_string(),
            Self::PDF => "http://127.0.0.1:58080".to_string(),
        }
    }

    pub async fn test(&self) -> Result<()> {
        let service = self.service();
        let url = Url::parse(service.as_str())?;
        let host = url.host_str().unwrap();
        let port = url.port().unwrap();
        match TcpListener::bind(format!("{}:{}", host, port)).await.log() {
            Ok(_) => Err(anyhow::anyhow!("Service Unavailable: {}", self.service()).log()),
            Err(_) => Ok(()),
        }
    }
}
