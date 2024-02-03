use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use reqwest::Body;
use serde::{Deserialize, Serialize};

use crate::{
    api::alist::Client,
    common::log::{LogJson, LogResult},
};

impl Client {
    pub async fn fs_put<T>(&self, path: &str, len: u64, body: T) -> Result<()>
    where
        T: Into<Body>,
    {
        dbg!(path, len);
        let request = self
            .client
            .put(format!("{}/api/fs/put", self.url))
            .header("Authorization", self.token.as_deref().unwrap())
            .header("File-Path", path)
            .header(
                "Content-Type",
                mime_guess::from_path(PathBuf::from_str(path).log()?)
                    .first_or_octet_stream()
                    .essence_str(),
            )
            .header("Content-Length", len)
            .body(body);
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response = response.json_log::<Response>().await?;
        crate::ensure!(response.code == 200);
        crate::ensure!(response.message == "success");
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// 状态码
    pub code: i64,
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
