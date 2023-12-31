use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Args;

use reqwest::Client;

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
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let items = self.args.items()?;
        let client = Client::new();
        let request = client
            .get(format!("{}/convert/sing-box", self.api))
            .query(&[(
                "sub",
                items
                    .iter()
                    .filter_map(|item| item.notes())
                    .collect::<Vec<_>>()
                    .join("|"),
            )])
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
