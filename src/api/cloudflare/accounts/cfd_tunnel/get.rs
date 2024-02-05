use anyhow::Result;

use crate::{
    api::cloudflare::ResponseArray,
    common::log::{LogJson, LogResult},
};

use super::{CfdTunnel, ClientCfdTunnel};

impl ClientCfdTunnel {
    pub async fn get(&self, name: Option<&str>) -> Result<Vec<CfdTunnel>> {
        let request = self
            .client
            .get(format!(
                "{}/accounts/{}/cfd_tunnel",
                self.api, self.account_id
            ))
            .bearer_auth(self.token.as_str());
        let request = if let Some(name) = name {
            request.query(&[("name", name)])
        } else {
            request
        };
        tracing::debug!("{:?}", request);
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response = response.json_log::<ResponseArray<CfdTunnel>>().await?.log();
        Ok(response.result)
    }
}
