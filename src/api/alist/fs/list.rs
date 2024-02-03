use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::alist::Client;
use crate::common::log::{LogJson, LogResult};

impl Client {
    pub async fn fs_list(&self, path: Option<&str>, refresh: Option<bool>) -> Result<Vec<Content>> {
        let mut content = vec![];
        for page in 0.. {
            let request = self
                .client
                .post(format!("{}/api/fs/list", self.url))
                .header("Authorization", self.token.as_deref().unwrap())
                .json(&Request {
                    page: Some(page),
                    password: None,
                    path: path.map(|s| s.to_string()),
                    per_page: None,
                    refresh: refresh,
                });
            let response = request.send().await.log()?;
            let response = response.error_for_status().log()?;
            let mut response = response.json_log::<Response>().await?;
            content.append(&mut response.data.content);
            if content.len() >= response.data.total as usize {
                break;
            }
        }
        Ok(content)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// 页数
    pub page: Option<i64>,
    /// 密码
    pub password: Option<String>,
    /// 路径
    pub path: Option<String>,
    /// 每页数目
    pub per_page: Option<i64>,
    /// 是否强制刷新
    pub refresh: Option<bool>,
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
    pub content: Vec<Content>,
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
