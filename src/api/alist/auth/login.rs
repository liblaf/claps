use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::alist::Client;
use crate::common::log::{LogJson, LogResult};

impl Client {
    pub async fn auth_login(self, username: &str, password: &str) -> Result<Self> {
        let request = self
            .client
            .post(format!("{}/api/auth/login", self.url))
            .json(&Request {
                otp_code: String::new(),
                password: password.to_string(),
                username: username.to_string(),
            });
        let response = request.send().await.log()?;
        let response = response.error_for_status().log()?;
        let response = response.json_log::<Response>().await.log()?;
        crate::ensure!(response.code == 200);
        crate::ensure!(response.message == "success");
        Ok(Self {
            token: Some(response.data.token),
            ..self
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    /// 二步验证码，二步验证码
    pub otp_code: String,
    /// 密码，密码
    pub password: String,
    /// 用户名，用户名
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// 状态码
    pub code: i64,
    /// data
    pub data: Data,
    /// 信息
    pub message: String,
}

/// data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// token
    pub token: String,
}
