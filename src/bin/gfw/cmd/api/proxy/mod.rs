use std::collections::HashMap;

use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;

use claps::api::clash::proxies::Proxy;
use claps::common::cmd::Run;

mod get;
mod set;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
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
            SubCmd::Get(cmd) => cmd.run().await,
            SubCmd::Set(cmd) => cmd.run().await,
        }
    }
}

fn pretty(proxies: &HashMap<String, Proxy>, name: &str) -> String {
    let proxy = proxies.get("PROXY").unwrap();
    let latency = crate::cmd::api::proxy::get_latency(&proxies, name);
    let mut output = String::new();
    output += name;
    if let Some(latency) = latency {
        output += format!(" ({})", latency).as_str();
    }
    let mut now = name;
    while let Some(n) = proxies.get(now).unwrap().now.as_deref() {
        output += format!(" -> {}", n).as_str();
        now = n;
    }
    if name == proxy.now.as_deref().unwrap() {
        output = output.bold().reversed().to_string();
    };
    if let Some(latency) = latency {
        if latency < 200 {
            output = output.green().to_string();
        } else if latency < 500 {
            output = output.yellow().to_string();
        } else {
            output = output.red().to_string();
        }
    }
    output
}

fn get_latency(proxies: &HashMap<String, Proxy>, name: &str) -> Option<u64> {
    let proxy = proxies.get(name)?;
    if let Some(history) = proxy.history.last() {
        return Some(history.delay);
    }
    if let Some(all) = proxy.all.as_deref() {
        for name in all {
            if let Some(latency) = get_latency(proxies, name) {
                return Some(latency);
            }
        }
    }
    None
}
