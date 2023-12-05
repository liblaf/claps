use anyhow::Result;
use clap::builder::styling::AnsiColor;
use clap::builder::Styles;

pub mod complete;

#[async_trait::async_trait]
pub trait Run {
    async fn run(self) -> Result<()>;
}

pub const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Blue.on_default().bold())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Magenta.on_default().bold());
