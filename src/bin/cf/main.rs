use anyhow::Result;
use clap::Parser;

mod cmd;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = cmd::Cmd::parse();
    cmd.run().await
}
