use std::{ffi::OsStr, process::Stdio};

use anyhow::Result;
use tokio::process::Command;

pub mod bw;
pub mod py;
pub mod service;

pub async fn run<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut args = args.into_iter();
    let mut cmd = Command::new(args.next().unwrap());
    let cmd = cmd
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let status = cmd.status().await?;
    crate::ensure!(status.success());
    Ok(())
}
