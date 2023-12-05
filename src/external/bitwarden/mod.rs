use std::ffi::OsStr;
use std::process::{Command, Stdio};

use anyhow::Result;
use serde::Deserialize;

use crate::common::log::LogResult;

fn bw<I, S>(command: &str, args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let cmd = &mut Command::new("bw");
    cmd.args(["--nointeraction", command])
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    tracing::debug!("{:?}", cmd);
    let output = cmd.output().log()?;
    anyhow::ensure!(output.status.success());
    String::from_utf8(output.stdout).log()
}

fn list<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    bw("list", args)
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub notes: Option<String>,
}

pub fn list_items<S>(folder: S) -> Result<Vec<Item>>
where
    S: AsRef<str>,
{
    serde_json::from_str(&list([
        "items",
        &format!("--folderid={}", folder.as_ref()),
    ])?)
    .log()
}

fn get<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    bw("get", args)
}

pub fn get_notes<S>(id: S) -> Result<String>
where
    S: AsRef<str>,
{
    get(["notes", id.as_ref()])
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
}

pub fn get_folder<S>(id: S) -> Result<Folder>
where
    S: AsRef<str>,
{
    serde_json::from_str(&get(["folder", id.as_ref()])?).log()
}
