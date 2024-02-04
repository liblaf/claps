use std::process::Stdio;

use anyhow::Result;
use clap::Args;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use claps::common::log::LogResult;
use claps::internal::sub::Sub;

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    urls: Vec<String>,
    #[arg(long)]
    no_country: bool,
    #[arg(short, long)]
    geo: bool,
    #[arg(short, long)]
    ipv: bool,
    #[arg(short, long, default_value_t = f64::NAN)]
    timeout: f64,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let mut cmd = Command::new("gfw-sub-conf");
        let cmd = cmd
            .env("COUNTRY", (!self.no_country).to_string())
            .env("GEO", self.geo.to_string())
            .env("IPV", self.ipv.to_string())
            .env("TIMEOUT", self.timeout.to_string());
        if let Some(level) = std::env::var_os("LOG_LEVEL") {
            cmd.env("LOGURU_LEVEL", level);
        }
        let subs = if self.urls.is_empty() {
            claps::internal::sub::get_subs().await?
        } else {
            self.urls.iter().map(Sub::from).collect()
        };
        let cmd = cmd.args(
            subs.iter()
                .filter_map(|s| s.url.as_ref())
                .map(|u| u.as_str()),
        );
        let cmd = cmd
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        tracing::debug!("{:?}", cmd);
        let child = cmd.spawn().log()?;
        let output = child.wait_with_output().await.log()?;
        claps::ensure!(output.status.success());
        let conf = output.stdout;
        tokio::io::stdout().write_all(conf.as_slice()).await.log()?;
        Ok(())
    }
}
