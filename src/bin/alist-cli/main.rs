use anyhow::Result;
use clap::Parser;

use claps::common::cmd::Run;

mod cmd;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = cmd::Cmd::parse();
    cmd.run().await?;
    Ok(())
}
