use std::marker::PhantomData;

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Args, CommandFactory, ValueEnum};
use clap_complete::Shell;

use crate::common::cmd::Run;

/// Generate tab-completion scripts for your shell
#[derive(Debug, Args)]
pub struct Cmd<C>
where
    C: CommandFactory + Send + Sync,
{
    shell: Generator,

    #[arg(skip)]
    phantom: PhantomData<C>,
}

#[derive(Clone, Debug)]
enum Generator {
    Markdown,
    Shell(Shell),
}

impl ValueEnum for Generator {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Markdown,
            Self::Shell(Shell::Bash),
            Self::Shell(Shell::Elvish),
            Self::Shell(Shell::Fish),
            Self::Shell(Shell::PowerShell),
            Self::Shell(Shell::Zsh),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Markdown => Some(PossibleValue::new("markdown")),
            Self::Shell(shell) => shell.to_possible_value(),
        }
    }
}

#[async_trait::async_trait]
impl<C> Run for Cmd<C>
where
    C: CommandFactory + Send + Sync,
{
    async fn run(self) -> Result<()> {
        match self.shell {
            Generator::Markdown => clap_markdown::print_help_markdown::<C>(),
            Generator::Shell(shell) => {
                let cmd = &mut C::command();
                clap_complete::generate(
                    shell,
                    cmd,
                    cmd.get_name().to_string(),
                    &mut std::io::stdout(),
                )
            }
        }
        Ok(())
    }
}
