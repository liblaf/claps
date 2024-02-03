use clap::builder::styling::{AnsiColor, Style};
use clap::builder::Styles;

pub mod complete;

pub const STYLES: Styles = Styles::styled()
    .error(AnsiColor::Red.on_default().bold())
    .header(AnsiColor::Green.on_default().bold())
    .invalid(AnsiColor::Yellow.on_default().bold())
    .literal(AnsiColor::Blue.on_default().bold())
    .placeholder(AnsiColor::Magenta.on_default().bold())
    .usage(AnsiColor::Cyan.on_default().bold())
    .valid(Style::new().bold());
