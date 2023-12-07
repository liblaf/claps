use anyhow::Result;
use clap::Args;
use directories::BaseDirs;

use claps::common::cmd::Run;
use claps::common::log::LogResult;

#[derive(Debug, Args)]
pub(super) struct Cmd {}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let base_dirs = BaseDirs::new().unwrap();
        let config_dir = base_dirs.config_dir();
        let contents = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ddns/ddns.service"
        ));
        let path = config_dir.join("systemd/user/ddns.service");
        std::fs::write(&path, contents).log()?;
        tracing::info!("installed: {:?}", path);
        let path = config_dir.join("systemd/user/ddns.timer");
        let contents = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/ddns/ddns.timer"));
        std::fs::write(&path, contents).log()?;
        tracing::info!("installed: {:?}", path);
        claps::external::run("systemctl", ["--user", "enable", "--now", "ddns.timer"])?;
        tracing::info!("systemctl --user enable --now ddns.timer");
        Ok(())
    }
}
