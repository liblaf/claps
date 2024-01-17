use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use colored::Colorize;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Format, Modify, Style};

use claps::common::cmd::{Run, STYLES};
use claps::common::log::LogInit;
use claps::external::bitwarden::types::Item;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, styles = STYLES)]
pub(super) struct Cmd {
    arg: Option<String>,

    #[command(subcommand)]
    sub_cmd: Option<SubCmd>,

    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd<Cmd>),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        self.verbosity.init();
        match self.sub_cmd {
            Some(cmd) => match cmd {
                SubCmd::Complete(cmd) => cmd.run().await,
            },
            None => {
                let search = self.arg.unwrap_or_default();
                let items = claps::external::bitwarden::list::items(&search, "")?;
                let mut builder = Builder::new();
                builder.set_header(["Name", "Username", "Password", "Notes"]);
                for item in items {
                    match item {
                        Item::Login(login) => {
                            builder.push_record([
                                login.name,
                                login.login.username.unwrap_or_default(),
                                login.login.password.unwrap_or_default(),
                                login.notes.unwrap_or_default(),
                            ]);
                        }
                        Item::SecureNote(note) => {
                            builder.push_record([
                                note.name,
                                String::new(),
                                String::new(),
                                note.notes,
                            ]);
                        }
                    }
                }
                let mut table = builder.build();
                table
                    .with(Style::rounded())
                    .with(
                        Modify::new(Columns::single(0))
                            .with(Format::content(|s| s.red().to_string())),
                    )
                    .with(
                        Modify::new(Columns::single(1))
                            .with(Format::content(|s| s.blue().to_string())),
                    )
                    .with(
                        Modify::new(Columns::single(2))
                            .with(Format::content(|s| s.green().to_string())),
                    )
                    .with(
                        Modify::new(Columns::single(3))
                            .with(Format::content(|s| s.cyan().to_string())),
                    );
                println!("{}", table);
                Ok(())
            }
        }
    }
}
