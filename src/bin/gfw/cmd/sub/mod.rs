use anyhow::Result;
use clap::{Args, Subcommand};

mod conf;
mod list;

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
}

#[derive(Subcommand)]
enum SubCmd {
    Conf(conf::Cmd),
    List(list::Cmd),
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self.sub_cmd {
            SubCmd::Conf(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
        }
    }
}
