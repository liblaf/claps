use std::ffi::{OsStr, OsString};
use std::process::Stdio;

use anyhow::Result;
use tokio::process::Command;

pub mod get;
pub mod list;
pub mod types;

async fn bw<I, S>(args: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let cmd = &mut Command::new("bw");
    let cmd = cmd.arg("--nointeraction").args(args);
    let cmd = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    tracing::debug!("{:?}", cmd);
    let output = cmd.output().await?;
    crate::ensure!(output.status.success());
    Ok(output.stdout)
}

async fn list<I, S>(args: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    bw(["list".parse::<OsString>()?]
        .into_iter()
        .chain(args.into_iter().map(|s| s.as_ref().to_os_string())))
    .await
}

async fn get<I, S>(args: I) -> Result<Vec<u8>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    bw(["get".parse::<OsString>()?]
        .into_iter()
        .chain(args.into_iter().map(|s| s.as_ref().to_os_string())))
    .await
}
