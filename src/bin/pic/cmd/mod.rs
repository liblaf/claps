use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::api::alist::Client;
use claps::common::cmd::STYLES;
use claps::common::log::LogInit;

mod list;
mod upload;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, styles = STYLES)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[command(flatten)]
    args: GlobalArgs,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd),
    List(list::Cmd),
    Upload(upload::Cmd),
}

#[derive(Args)]
struct GlobalArgs {
    #[arg(long, default_value = "https://alist.liblaf.me", global = true)]
    url: String,
    #[arg(short, long, global = true)]
    username: Option<String>,
    #[arg(short, long, global = true)]
    password: Option<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::List(cmd) => cmd.run(self.args).await,
            SubCmd::Upload(cmd) => cmd.run(self.args).await,
        }
    }
}

impl GlobalArgs {
    async fn username(&self) -> Result<String> {
        if let Some(username) = self.username.as_deref() {
            return Ok(username.to_string());
        }
        claps::external::bw::get::username(self.url.as_str()).await
    }

    async fn password(&self) -> Result<String> {
        if let Some(password) = self.password.as_deref() {
            return Ok(password.to_string());
        }
        claps::external::bw::get::password(self.url.as_str()).await
    }

    async fn client(&self) -> Result<Client> {
        let username = self.username().await?;
        let password = self.password().await?;
        let client = Client::new(self.url.as_str());
        client
            .auth_login(username.as_str(), password.as_str())
            .await
    }
}
