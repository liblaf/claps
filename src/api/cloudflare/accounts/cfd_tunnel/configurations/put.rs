use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::common::log::LogResult;

use super::{ClientConfigurations, Config};

impl ClientConfigurations {
    pub async fn put(&self, config: &Config) -> Result<()> {
        let request = self
            .client
            .put(format!(
                "{}/accounts/{}/cfd_tunnel/{}/configurations",
                self.api, self.account_id, self.tunnel_id
            ))
            .bearer_auth(self.token.as_str())
            .json(&Tunnel {
                config: config.to_owned(),
            });
        let response = request.send().await.log()?;
        crate::api::cloudflare::handle::<Tunnel>(response).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tunnel {
    config: Config,
}
