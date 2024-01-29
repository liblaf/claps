use anyhow::Result;
use serde::Deserialize;

use crate::api::alist::{Client, JsonOrLog};
use crate::common::log::LogResult;

impl Client {
    pub async fn auth_login(&mut self, username: &str, password: &str) -> Result<()> {
        let url = format!("{}/auth/login", self.url);
        let request = self.client.post(&url);
        let request = request.json(&serde_json::json!({
            "username": username,
            "password": password,
        }));
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json_or_log().await?;
        crate::ensure!(response.code == 200);
        crate::ensure!(response.message == "success");
        self.token = Some(response.data.token);
        Ok(())
    }
}

type Code = u16;

#[derive(Debug, Deserialize)]
struct Response {
    code: Code,
    message: String,
    data: ResponseData,
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    token: String,
}
