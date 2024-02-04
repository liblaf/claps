use anyhow::Result;

use crate::common::log::LogResult;
use crate::external::bw::types::Folder;

pub async fn username(id: &str) -> Result<String> {
    String::from_utf8(crate::external::bw::get(["username", id]).await?).log()
}

pub async fn password(id: &str) -> Result<String> {
    String::from_utf8(crate::external::bw::get(["password", id]).await?).log()
}

pub async fn notes(id: &str) -> Result<String> {
    String::from_utf8(crate::external::bw::get(["notes", id]).await?).log()
}

pub async fn folder(id: &str) -> Result<Folder> {
    serde_json::from_slice(crate::external::bw::get(["folder", id]).await?.as_slice()).log()
}
