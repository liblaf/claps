use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::peaker::PriorityMax;
use tabled::settings::{Color, Style, Width};

use claps::cmd::bw::types::Item;
use claps::common::cmd::STYLES;
use claps::common::log::{LogInit, LogResult};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, styles = STYLES)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: Option<SubCmd>,
    #[arg()]
    search: Option<String>,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd),
}

impl Cmd {
    pub async fn run(self) -> anyhow::Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            Some(SubCmd::Complete(cmd)) => cmd.run(Cmd::command()),
            None => {
                let mut builder = Builder::new();
                builder.push_record(["Name", "Username", "Password", "Notes"]);
                let items = claps::cmd::bw::list::items(self.search.as_deref()).await?;
                for item in items {
                    match item {
                        Item::Login(item) => {
                            builder.push_record([
                                item.name,
                                item.login.username.unwrap_or_default(),
                                item.login.password.unwrap_or_default(),
                                item.notes.unwrap_or_default(),
                            ]);
                        }
                        Item::SecureNote(item) => {
                            builder.push_record([
                                item.name,
                                String::new(),
                                String::new(),
                                item.notes.unwrap_or_default(),
                            ]);
                        }
                    }
                }
                let mut table = builder.build();
                let (width, _) = crossterm::terminal::size().log()?;
                table
                    .with(Style::rounded())
                    .with(
                        Width::truncate(width as usize)
                            .priority::<PriorityMax>()
                            .suffix("..."),
                    )
                    .modify(Columns::single(0), Color::FG_RED)
                    .modify(Columns::single(1), Color::FG_GREEN)
                    .modify(Columns::single(2), Color::FG_YELLOW)
                    .modify(Columns::single(3), Color::FG_BLUE);
                println!("{}", table);
                Ok(())
            }
        }
    }
}
