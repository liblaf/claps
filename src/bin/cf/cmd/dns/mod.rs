use anyhow::Result;
use clap::{Args, Subcommand};

mod delete;
mod install;
mod list;
mod update;

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self.cmd {
            SubCmd::Delete(cmd) => cmd.run().await,
            SubCmd::Install(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Update(cmd) => cmd.run().await,
        }
    }
}

#[derive(Subcommand)]
enum SubCmd {
    Delete(delete::Cmd),
    Install(install::Cmd),
    List(list::Cmd),
    Update(update::Cmd),
}
