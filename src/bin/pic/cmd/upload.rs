use std::path::PathBuf;

use anyhow::Result;
use chrono::Utc;
use clap::Args;
use reqwest::Url;
use tokio::fs::File;

use claps::common::log::LogResult;

use crate::cmd::GlobalArgs;

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    file: PathBuf,
    #[arg(long, default_value = "/public/img")]
    prefix: PathBuf,
}

impl Cmd {
    pub async fn run(self, args: GlobalArgs) -> Result<()> {
        let client = args.client().await?;
        let now = Utc::now();
        let path = self
            .prefix
            .join(now.format("%Y").to_string())
            .join(now.format("%FT%H%M%S").to_string());
        let path = if let Some(ext) = self.file.extension() {
            path.with_extension(ext)
        } else {
            path
        };
        client
            .fs_put(
                path.to_str().unwrap(),
                self.file.metadata().log()?.len(),
                File::open(self.file.as_path()).await.log()?,
            )
            .await?;
        tracing::info!("Upload: {} -> {}", self.file.display(), path.display());
        let cdn_path = "/img"
            .parse::<PathBuf>()?
            .join(now.format("%Y").to_string())
            .join(now.format("%m").to_string())
            .join(now.format("%d").to_string())
            .join(path.file_name().unwrap());
        let mut cdn_url = "https://cdn.liblaf.me".parse::<Url>()?;
        cdn_url.set_path(cdn_path.to_str().unwrap());
        println!("{}", cdn_url);
        Ok(())
    }
}
