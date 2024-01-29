use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::{Run, STYLES};
use claps::common::log::LogInit;

mod list;
mod remove;
mod upload;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, styles = STYLES)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[command(flatten)]
    args: CommonArgs,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Args)]
struct CommonArgs {
    #[arg(long, default_value = "https://alist.liblaf.me/api", global = true)]
    url: String,
    #[arg(short, long, global = true)]
    username: Option<String>,
    #[arg(short, long, global = true)]
    password: Option<String>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd<Cmd>),
    List(list::Cmd),
    Upload(upload::Cmd),
    Remove(remove::Cmd),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run().await?,
            SubCmd::List(cmd) => cmd.run(self.args).await?,
            SubCmd::Upload(cmd) => cmd.run(self.args).await?,
            SubCmd::Remove(cmd) => cmd.run(self.args).await?,
        }
        Ok(())
    }
}

impl CommonArgs {
    fn username(&self) -> Result<String> {
        if let Some(username) = self.username.as_deref() {
            return Ok(username.to_string());
        }
        claps::external::bitwarden::get::username("alist.liblaf.me")
    }

    fn password(&self) -> Result<String> {
        if let Some(password) = self.password.as_deref() {
            return Ok(password.to_string());
        }
        claps::external::bitwarden::get::password("alist.liblaf.me")
    }
}
