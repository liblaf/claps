use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use claps::api::alist::Client;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    path: Vec<PathBuf>,
    #[arg(long)]
    no_refresh: bool,
}

impl Cmd {
    pub async fn run(self, args: crate::cmd::CommonArgs) -> Result<()> {
        let mut client = Client::new(args.url.as_str());
        client
            .auth_login(args.username()?.as_str(), args.password()?.as_str())
            .await?;
        for path in self.path {
            let dir = match path.parent() {
                Some(dir) => dir.to_str().unwrap(),
                None => "/",
            };
            client
                .fs_remove([path.file_name().unwrap().to_str().unwrap()], dir)
                .await?;
            tracing::info!("Remove: {}", path.display());
            if !self.no_refresh {
                client.fs_list(Some(dir), None, Some(true)).await?;
            }
        }
        Ok(())
    }
}
