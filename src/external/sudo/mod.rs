use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::Result;

use crate::common::log::LogResult;

pub fn sudo<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new("sudo");
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    tracing::debug!("{:?}", cmd);
    let status = cmd.status().log()?;
    crate::ensure!(status.success());
    Ok(())
}

pub fn tee<P, C>(path: P, contents: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let mut cmd = Command::new("sudo");
    cmd.arg("tee")
        .arg(path.as_ref())
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit());
    tracing::debug!("{:?}", cmd);
    let mut child = cmd.spawn().log()?;
    child
        .stdin
        .as_ref()
        .unwrap()
        .write_all(contents.as_ref())
        .log()?;
    let status = child.wait().log()?;
    crate::ensure!(status.success());
    Ok(())
}
