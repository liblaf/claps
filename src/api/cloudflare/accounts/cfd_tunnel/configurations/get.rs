use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    api::cloudflare::ResponseObject,
    common::log::{LogJson, LogResult},
};

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
        let response = response.error_for_status().log()?;
        let response = response
            .json_log::<ResponseObject<ResponseResult>>()
            .await?
            .log();
        Ok(response.result.config)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseResult {
    config: Config,
}
