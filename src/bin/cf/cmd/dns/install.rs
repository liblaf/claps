use anyhow::Result;
use clap::Args;
use directories::BaseDirs;

use claps::common::log::LogResult;

#[derive(Args)]
pub struct Cmd {}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let dirs = BaseDirs::new().unwrap();
        let dir = dirs.config_dir();
        let contents = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/",
            env!("CARGO_BIN_NAME"),
            "/ddns.service"
        ));
        let path = dir.join("systemd/user/ddns.service");
        tokio::fs::write(path.as_path(), contents).await.log()?;
        tracing::info!("installed: {:?}", path);
        let path = dir.join("systemd/user/ddns.timer");
        let contents = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/",
            env!("CARGO_BIN_NAME"),
            "/ddns.timer"
        ));
        tokio::fs::write(&path, contents).await.log()?;
        tracing::info!("installed: {:?}", path);
        claps::external::run(["systemctl", "--user", "enable", "--now", "ddns.timer"]).await?;
        tracing::info!("systemctl --user enable --now ddns.timer");
        Ok(())
    }
}
