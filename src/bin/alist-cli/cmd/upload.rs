use std::path::PathBuf;

use anyhow::Result;
use chrono::{Datelike, Utc};
use clap::Args;

use claps::api::alist::Client;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    path: PathBuf,
}

impl Cmd {
    pub async fn run(self, args: crate::cmd::CommonArgs) -> Result<()> {
        let mut client = Client::new(args.url.as_str());
        client
            .auth_login(args.username()?.as_str(), args.password()?.as_str())
            .await?;
        let body = std::fs::read(self.path.as_path())?;
        let time = Utc::now();
        let file_path = PathBuf::from_iter(
            [
                "public",
                "img",
                time.year().to_string().as_str(),
                time.format("%F-%H%M%S").to_string().as_str(),
            ]
            .iter(),
        );
        let file_path = if let Some(extension) = self.path.extension() {
            let extension = extension.to_str().unwrap();
            file_path.with_extension(extension)
        } else {
            file_path
        };
        let content_type = mime_guess::from_path(self.path.as_path()).first_or_octet_stream();
        client
            .fs_put(
                file_path.as_path(),
                content_type.essence_str(),
                body.len(),
                body,
            )
            .await?;
        tracing::info!("Upload: {} -> {}", self.path.display(), file_path.display());
        Ok(())
    }
}
