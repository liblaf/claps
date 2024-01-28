use std::panic::Location;
use std::str::FromStr;

use clap_verbosity_flag::{LogLevel, Verbosity};

pub trait LogInit {
    fn init(self);
}

pub trait LogError {
    #[track_caller]
    fn log(self) -> anyhow::Error;
}

pub trait LogResult<T> {
    #[track_caller]
    fn log(self) -> anyhow::Result<T>;
}

impl<L> LogInit for Verbosity<L>
where
    L: LogLevel,
{
    fn init(self) {
        if let Some(level) = self.log_level() {
            let level = tracing::Level::from_str(level.as_str()).unwrap();
            let builder = tracing_subscriber::fmt().pretty().with_max_level(level);
            if level < tracing::Level::DEBUG {
                builder
                    .with_file(false)
                    .with_line_number(false)
                    .with_target(false)
                    .without_time()
                    .init();
            } else {
                builder.init();
            }
        }
    }
}

impl<E> LogError for E
where
    E: Into<anyhow::Error>,
{
    #[track_caller]
    fn log(self) -> anyhow::Error {
        let e = self.into();
        let mut message = e.to_string();
        let sources = e
            .chain()
            .skip(1)
            .enumerate()
            .map(|(i, e)| format!("{:>5}: {}", i, e))
            .collect::<Vec<_>>()
            .join("\n");
        if !sources.is_empty() {
            message += "\nCaused by:\n";
            message += &sources;
            message += "\n";
        }
        let location = Location::caller();
        tracing::error!(
            { location = format!("{}:{}", location.file(), location.line()) },
            "{}",
            message
        );
        e
    }
}

impl<T, E> LogResult<T> for std::result::Result<T, E>
where
    E: Into<anyhow::Error>,
{
    #[track_caller]
    fn log(self) -> anyhow::Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.log()),
        }
    }
}
