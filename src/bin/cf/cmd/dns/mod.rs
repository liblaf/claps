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
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(
        short,
        long,
        default_value = "919b04037636d3b4bbc0af135eaccdfa",
        global(true)
    )]
    zone: String,
    #[arg(short, long, global(true))]
    name: Option<String>,
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
