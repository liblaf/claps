use anyhow::Result;
use clap::Args;

use claps::common::cmd::Run;
use claps::external::bitwarden::types::Item;
use colored::Colorize;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Format, Modify, Style};

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[arg(default_value = "")]
    search: String,
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let items = claps::external::bitwarden::list::items(&self.search, "")?;
        let mut builder = Builder::new();
        builder.set_header(["Name", "Username", "Password"]);
        for item in items {
            match item {
                Item::Login(login) => {
                    builder.push_record([
                        login.name,
                        login.login.username.unwrap_or_default(),
                        login.login.password.unwrap_or_default(),
                    ]);
                }
                Item::SecureNote(_) => {}
            }
        }
        let mut table = builder.build();
        table
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Format::content(|s| s.red().to_string())))
            .with(Modify::new(Columns::single(1)).with(Format::content(|s| s.blue().to_string())))
            .with(Modify::new(Columns::single(2)).with(Format::content(|s| s.green().to_string())));
        println!("{}", table);
        Ok(())
    }
}
