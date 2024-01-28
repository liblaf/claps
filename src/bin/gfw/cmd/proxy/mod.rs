use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;
use reqwest::Url;

use claps::api::clash::Client;
use claps::common::cmd::Run;

mod get;
mod set;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[command(flatten)]
    args: CommonArgs,
}

#[derive(Debug, Args)]
struct CommonArgs {
    #[arg(short, long, default_value = "http://127.0.0.1:9090", global = true)]
    url: Url,
    #[arg(short, long, global = true)]
    secret: Option<String>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Get(get::Cmd),
    Set(set::Cmd),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        match self.sub_cmd {
            SubCmd::Get(cmd) => cmd.run(self.args).await,
            SubCmd::Set(cmd) => cmd.run(self.args).await,
        }
    }
}

async fn pretty(client: &Client, group: &str) -> Result<Vec<String>> {
    let proxies = client.proxies().await?;
    let delay = client.group_delay(group).await.unwrap_or_default();
    let group = proxies.get(group).unwrap();
    Ok(group
        .all
        .as_deref()
        .unwrap()
        .iter()
        .map(|name| {
            let mut output = name.to_string();
            let delay = delay.get(name).map(|delay| delay.to_owned()).or_else(|| {
                proxies
                    .get(name)
                    .unwrap()
                    .history
                    .last()
                    .map(|history| history.delay)
            });
            if let Some(delay) = delay {
                output += format!(" ({})", delay).as_str();
            }
            let mut now = proxies.get(name).unwrap().now.as_deref();
            while let Some(next) = now {
                output += format!(" -> {}", next).as_str();
                now = proxies.get(next).unwrap().now.as_deref();
            }
            if name == group.now.as_deref().unwrap() {
                output = output.bold().reversed().to_string();
            }
            if let Some(delay) = delay {
                if delay < 200 {
                    output = output.green().to_string();
                } else if delay < 500 {
                    output = output.yellow().to_string();
                } else {
                    output = output.red().to_string();
                }
            }
            output
        })
        .collect())
}
