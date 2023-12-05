use std::process::{Command, Stdio};

use anyhow::Result;

use crate::common::log::LogResult;

pub fn get_notes<S>(id: S) -> Result<String>
where
    S: AsRef<str>,
{
    let cmd = &mut Command::new("bw");
    cmd.args(["--nointeraction", "get", "notes", id.as_ref()])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    tracing::debug!("{:?}", cmd);
    let output = cmd.output().log()?;
    anyhow::ensure!(output.status.success());
    String::from_utf8(output.stdout).log()
}
