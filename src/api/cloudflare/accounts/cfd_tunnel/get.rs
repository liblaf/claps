use anyhow::Result;

use crate::common::log::LogResult;

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
        let result = crate::api::cloudflare::handle::<Vec<CfdTunnel>>(response).await?;
        Ok(result)
    }
}
