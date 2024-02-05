use std::fmt::{Display, Formatter};

use anyhow::Result;
use clap::ValueEnum;

use crate::{
    api::cloudflare::accounts::cfd_tunnel::configurations::Ingress,
    common::log::{LogError, LogResult},
};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Service {
    Alist,
    Bt,
    Dns,
    Glances,
    Gpt,
    Jellyfin,
    Pdf,
}

impl Display for Service {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}

impl Service {
    pub fn hostname(&self, name: Option<&str>) -> String {
        let name = name.map(|s| s.to_lowercase());
        format!("{}-{}.liblaf.me", self, name.unwrap_or(whoami::hostname()))
    }

    pub fn service(&self) -> String {
        match self {
            Self::Alist => "http://127.0.0.1:5244".to_string(),
            Self::Bt => "http://127.0.0.1:8000".to_string(),
            Self::Dns => "tcp://127.0.0.1:60053".to_string(),
            Self::Glances => "http://127.0.0.1:61208".to_string(),
            Self::Gpt => "http://127.0.0.1:12303".to_string(),
            Self::Jellyfin => "http://127.0.0.1:8096".to_string(),
            Self::Pdf => "http://127.0.0.1:58080".to_string(),
        }
    }

    pub fn ingress(&self, name: Option<&str>) -> Ingress {
        Ingress {
            hostname: Some(self.hostname(name)),
            service: self.service(),
        }
    }

    pub fn ingress_main(&self) -> Ingress {
        Ingress {
            hostname: Some(format!("{}.liblaf.me", self)),
            service: self.service(),
        }
    }

    pub async fn test(&self) -> Result<()> {
        let service = self.service();
        match reqwest::get(service).await.log() {
            Ok(_) => {
                tracing::debug!("Service Available: {}", self.service());
                Ok(())
            }
            Err(_) => Err(anyhow::anyhow!("Service Unavailable: {}", self.service()).log()),
        }
    }
}
