use anyhow::Result;
use clap::Args;

use claps::common::log::LogResult;
use claps::internal::singbox::conf::Config;

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    urls: Vec<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let conf = Config::default();
        serde_json::to_writer_pretty(&mut std::io::stdout(), &conf).log()?;
        todo!()
    }
}
