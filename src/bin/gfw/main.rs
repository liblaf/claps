use anyhow::Result;
use clap::Parser;

use claps::common::cmd::Run;

mod cmd;
pub mod fmt;
pub mod sub;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = cmd::Cmd::parse();
    cmd.run().await
}
