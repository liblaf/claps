use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::common::log::LogResult;

use super::{ClientConfigurations, Config};

impl ClientConfigurations {
    pub async fn get(&self) -> Result<Config> {
        let request = self
            .client
            .get(format!(
                "{}/accounts/{}/cfd_tunnel/{}/configurations",
                self.api, self.account_id, self.tunnel_id
            ))
            .bearer_auth(self.token.as_str());
        let response = request.send().await.log()?;
        let result = crate::api::cloudflare::handle::<Tunnel>(response).await?;
        Ok(result.config)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tunnel {
    config: Config,
}
