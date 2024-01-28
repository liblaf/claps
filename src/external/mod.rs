use std::ffi::OsStr;
use std::process::{Command, Stdio};

use anyhow::Result;

use crate::common::log::LogResult;

pub mod bitwarden;
pub mod sudo;

pub fn run<I, S>(program: S, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(program);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    tracing::debug!("{:?}", cmd);
    let status = cmd.status().log()?;
    crate::ensure!(status.success());
    Ok(())
}
