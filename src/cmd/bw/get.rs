use anyhow::Result;

use crate::common::log::LogResult;

pub async fn username(id: &str) -> Result<String> {
    String::from_utf8(crate::cmd::bw::get(["username", id]).await?).log()
}

pub async fn password(id: &str) -> Result<String> {
    String::from_utf8(crate::cmd::bw::get(["password", id]).await?).log()
}

pub async fn notes(id: &str) -> Result<String> {
    String::from_utf8(crate::cmd::bw::get(["notes", id]).await?).log()
}
