use std::path::Path;

use anyhow::Result;
use reqwest::Body;
use serde::{Deserialize, Serialize};

use crate::api::alist::{Client, JsonOrLog};
use crate::common::log::LogResult;

impl Client {
    pub async fn fs_put<T>(
        &self,
        file_path: &Path,
        content_type: &str,
        content_length: usize,
        body: T,
    ) -> Result<()>
    where
        T: Into<Body>,
    {
        let url = format!("{}/fs/put", self.url);
        let body = body.into();
        let request = self.client.put(&url);
        let request = request
            .header("Authorization", self.token()?)
            .header("File-Path", file_path.to_str().unwrap())
            .header("Content-Type", content_type)
            .header("Content-Length", content_length);
        let request = request.body(body);
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json_or_log().await?;
        crate::ensure!(response.code == 200);
        crate::ensure!(response.message == "success");
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// 状态码
    pub code: i64,
    // pub data: Data,
    pub data: Option<Data>,
    /// 信息
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub task: Task,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub error: String,
    pub id: String,
    pub name: String,
    pub progress: i64,
    pub state: i64,
    pub status: String,
}
