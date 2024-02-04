use std::panic::Location;

use clap_verbosity_flag::Verbosity;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub trait LogInit {
    fn init(&self);
}

impl<L> LogInit for Verbosity<L>
where
    L: clap_verbosity_flag::LogLevel,
{
    fn init(&self) {
        if let Some(level) = self.log_level() {
            std::env::set_var("LOG_LEVEL", level.to_string());
            let builder = tracing_subscriber::fmt().with_writer(std::io::stderr);
            let builder = match level {
                clap_verbosity_flag::Level::Error => builder.with_max_level(tracing::Level::ERROR),
                clap_verbosity_flag::Level::Warn => builder.with_max_level(tracing::Level::WARN),
                clap_verbosity_flag::Level::Info => builder.with_max_level(tracing::Level::INFO),
                clap_verbosity_flag::Level::Debug => builder.with_max_level(tracing::Level::DEBUG),
                clap_verbosity_flag::Level::Trace => builder.with_max_level(tracing::Level::TRACE),
            };
            if level < clap_verbosity_flag::Level::Debug {
                builder.without_time().init();
            } else {
                builder.init();
            }
        }
    }
}

pub trait LogError<E> {
    #[track_caller]
    fn log(self) -> anyhow::Error;
}

impl<E> LogError<E> for E
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
            message += sources.as_str();
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

pub trait LogResult<T> {
    #[track_caller]
    fn log(self) -> anyhow::Result<T>;
}

impl<T, E> LogResult<T> for Result<T, E>
where
    E: LogError<E>,
{
    #[track_caller]
    fn log(self) -> anyhow::Result<T> {
        match self {
            Ok(result) => Ok(result),
            Err(e) => Err(e.log()),
        }
    }
}

pub trait LogJson {
    #[track_caller]
    async fn json_log<T>(self) -> anyhow::Result<T>
    where
        T: DeserializeOwned;
}

impl LogJson for Response {
    #[track_caller]
    async fn json_log<T>(self) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let text = self.text().await.log()?;
        match serde_json::from_str::<T>(text.as_str()).log() {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::error!({ text = text.as_str() }, "{}", e);
                Err(e)
            }
        }
    }
}
