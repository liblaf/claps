use anyhow::Result;
use clap::Args;

use claps::common::cmd::Run;
use claps::common::log::LogResult;

use crate::fmt::{Field, FmtTable};
use crate::sub::Sub;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(flatten)]
    args: super::CommonArgs,

    #[arg(short, long, value_enum, default_values_t = FIELDS)]
    fields: Vec<Field>,
}

const FIELDS: &[Field] = &[
    Field::Name,
    Field::Download,
    Field::Upload,
    Field::Total,
    Field::Expire,
];

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let items = self.args.items()?;
        let subs: Vec<Sub> = futures::future::join_all(items.iter().map(|item| Sub::from(item)))
            .await
            .into_iter()
            .filter_map(|sub| sub.log().ok())
            .collect();
        let table = subs.fmt_table(&self.fields);
        println!("{}", table);
        Ok(())
    }
}
