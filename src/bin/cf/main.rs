use anyhow::Result;
use clap::Parser;

mod cmd;
pub mod helper;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = cmd::Cmd::parse();
    cmd.run().await
}
