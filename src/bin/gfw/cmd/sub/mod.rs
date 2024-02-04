use anyhow::Result;
use clap::{Args, Subcommand};

mod list;

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
}

#[derive(Subcommand)]
enum SubCmd {
    List(list::Cmd),
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self.sub_cmd {
            SubCmd::List(cmd) => cmd.run().await,
        }
    }
}
