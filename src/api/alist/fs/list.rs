use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::alist::{Client, JsonOrLog};
use crate::common::log::LogResult;

impl Client {
    pub async fn fs_list(
        &self,
        path: Option<&str>,
        password: Option<&str>,
        refresh: Option<bool>,
    ) -> Result<Data> {
        let url = format!("{}/fs/list", self.url);
        let request = self.client.post(&url);
        let request = self.auth(request)?;
        let request = request.json(&Request {
            page: None,
            password: password.map(|s| s.to_string()),
            path: path.map(|s| s.to_string()),
            per_page: Some(i64::MAX),
            refresh,
        });
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response: Response = response.json_or_log().await?;
        crate::ensure!(response.code == 200);
        crate::ensure!(response.message == "success");
        Ok(response.data)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// 页数
    page: Option<i64>,
    /// 密码
    password: Option<String>,
    /// 路径
    path: Option<String>,
    /// 每页数目
    per_page: Option<i64>,
    /// 是否强制刷新
    refresh: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// 状态码
    pub code: i64,
    pub data: Data,
    /// 信息
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// 内容
    // pub content: Vec<Content>,
    pub content: Option<Vec<Content>>,
    pub provider: String,
    /// 说明
    pub readme: String,
    /// 总数
    pub total: i64,
    /// 是否可写入
    pub write: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    /// 是否是文件夹
    pub is_dir: bool,
    /// 修改时间
    pub modified: String,
    /// 文件名
    pub name: String,
    /// 签名
    pub sign: String,
    /// 大小
    pub size: i64,
    /// 缩略图
    pub thumb: String,
    /// 类型
    #[serde(rename = "type")]
    pub content_type: i64,
}
