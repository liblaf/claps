use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::Run;
use claps::common::log::LogInit;

mod delete;
mod list;
mod update;

#[derive(Debug, Parser)]
#[command(version, author)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Delete(delete::Cmd),
    List(list::Cmd),
    Update(update::Cmd),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Delete(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Update(cmd) => cmd.run().await,
        }
    }
}
