use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;
use clap::Args;

use reqwest::{Client, ClientBuilder, IntoUrl, Response};

use claps::common::cmd::Run;
use claps::common::log::LogResult;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(flatten)]
    args: super::CommonArgs,
    #[arg(short, long, default_value = "https://gfw.liblaf.me")]
    api: String,
    #[arg(short, long, default_value = "/etc/sing-box/config.json")]
    config: PathBuf,
    #[arg(short, long)]
    dns: Option<String>,
    #[arg(short, long, default_value_t = false)]
    tun: bool,
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let items = self.args.items()?;
        let client = Client::new();
        let request = client
            .get(format!("{}/convert/sing-box", self.api))
            .query(&[
                (
                    "sub",
                    items
                        .iter()
                        .filter_map(|item| item.notes())
                        .collect::<Vec<_>>()
                        .join("|"),
                ),
                ("dns", self.dns().await?),
                ("tun", self.tun.to_string()),
            ])
            .build()?;
        tracing::info!("{}", request.url());
        let response = client
            .execute(request)
            .await
            .log()?
            .error_for_status()
            .log()?;
        try_write(self.config, response.bytes().await.log()?)?;
        claps::external::sudo::sudo(["systemctl", "restart", "sing-box.service"])?;
        tracing::info!("sudo systemctl restart sing-box.service");
        Ok(())
    }
}

impl Cmd {
    async fn dns(&self) -> Result<String> {
        if let Some(dns) = self.dns.as_deref() {
            return Ok(dns.to_string());
        }
        match try_connect("https://101.6.6.6").await {
            Ok(_) => Ok("tuna".to_string()),
            Err(_) => Ok("alidns".to_string()),
        }
    }
}

fn try_write<P, C>(path: P, contents: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let path = path.as_ref();
    let contents = contents.as_ref();
    if let Ok(()) = std::fs::write(path, contents).log() {
        tracing::info!("write {:?}", path);
        return Ok(());
    }
    claps::external::sudo::tee(path, contents)?;
    tracing::info!("sudo tee {:?}", path);
    Ok(())
}

async fn try_connect<U>(url: U) -> Result<Response>
where
    U: IntoUrl,
{
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(1))
        .build()
        .log()?;
    let request = client.get(url).build().log()?;
    let response = client.execute(request).await.log()?;
    let response = response.error_for_status().log()?;
    Ok(response)
}
