use std::ffi::OsStr;
use std::process::{Command, Stdio};

use anyhow::Result;

use crate::common::log::LogResult;

pub mod get;
pub mod list;
pub mod types;

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
