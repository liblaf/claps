use std::ffi::OsStr;

use anyhow::Result;

use crate::common::log::LogResult;
use crate::external::bitwarden::types::Folder;

fn get<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    crate::external::bitwarden::bw("get", args)
}

pub fn username(id: &str) -> Result<String> {
    get(["username", id])
}

pub fn password(id: &str) -> Result<String> {
    get(["password", id])
}

pub fn notes(id: &str) -> Result<String> {
    get(["notes", id])
}

pub fn folder(id: &str) -> Result<Folder> {
    serde_json::from_str(&get(["folder", id])?).log()
}
