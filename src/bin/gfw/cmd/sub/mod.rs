use anyhow::Result;
use clap::{Args, Subcommand};

use claps::common::cmd::Run;
use claps::external::bitwarden::types::Item;

mod list;
mod update;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    List(list::Cmd),
    Update(update::Cmd),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        match self.sub_cmd {
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Update(cmd) => cmd.run().await,
        }
    }
}

#[derive(Debug, Args)]
struct CommonArgs {
    #[arg(long, default_value = "the Great Wall")]
    folder: String,
}

impl CommonArgs {
    fn items(&self) -> Result<Vec<Item>> {
        let folder = claps::external::bitwarden::get::folder(&self.folder)?;
        claps::external::bitwarden::list::items("", &folder.id)
    }
}
