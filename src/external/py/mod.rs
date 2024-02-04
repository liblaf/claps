use std::net::IpAddr;
use std::process::Stdio;

use anyhow::Result;
use tokio::process::Command;

use crate::common::log::LogResult;

pub async fn ip() -> Result<Vec<IpAddr>> {
    let mut cmd = Command::new("python3");
    cmd.arg("-c")
        .arg(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/scripts/ip.py"
        )))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    let child = cmd.spawn().log()?;
    let output = child.wait_with_output().await.log()?;
    crate::ensure!(output.status.success());
    let output = String::from_utf8(output.stdout).log()?;
    let addrs = output
        .lines()
        .map(|line| line.parse::<IpAddr>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(addrs)
}
