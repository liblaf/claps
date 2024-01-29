use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::alist::{Client, JsonOrLog};
use crate::common::log::LogResult;

impl Client {
    pub async fn fs_remove<I, S>(&self, names: I, dir: &str) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let request = self.client.post(format!("{}/fs/remove", self.url));
        let request = request.header("Authorization", self.token()?);
        let request = request.json(&Request {
            dir: dir.into(),
            names: names.into_iter().map(|s| s.as_ref().into()).collect(),
        });
        let response = request.send().await.log()?;
        let response: Response = response.json_or_log().await?;
        crate::ensure!(response.code == 200);
        crate::ensure!(response.message == "success");
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// 目录
    pub dir: String,
    /// 文件名
    pub names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// 状态码
    pub code: i64,
    pub data: Option<()>,
    /// 信息
    pub message: String,
}
