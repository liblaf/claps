use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use clap::Args;
use colored::Colorize;
use indicatif::HumanBytes;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Alignment, Color, Style};

use claps::internal::sub::Sub;

#[derive(Args)]
pub struct Cmd {
    #[arg()]
    urls: Vec<String>,
    #[arg(short, long, default_value_t = false)]
    url: bool,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let subs = if self.urls.is_empty() {
            claps::internal::sub::get_subs().await?
        } else {
            self.urls.into_iter().map(Sub::from).collect()
        };
        let subs = futures::future::join_all(subs.into_iter().map(Sub::with_info)).await;
        let mut table = Builder::new();
        let table = if self.url {
            table.push_record(["Name", "URL"]);
            for sub in subs.as_slice() {
                table.push_record([
                    sub.name.as_str(),
                    sub.url.as_ref().map(|u| u.as_str()).unwrap_or_default(),
                ])
            }
            let mut table = table.build();
            table
                .with(Style::rounded())
                .modify(Columns::single(0), Color::FG_BLUE);
            table
        } else {
            table.push_record(["Name", "Upload", "Download", "Remain", "Expire"]);
            for sub in subs.as_slice() {
                if let Some(info) = sub.user_info.as_ref() {
                    let usage = info.download + info.upload;
                    let ratio = usage as f64 / info.total as f64;
                    let color_bytes = if ratio < 0.6 {
                        colored::Color::Green
                    } else if ratio < 0.8 {
                        colored::Color::Yellow
                    } else {
                        colored::Color::Red
                    };
                    let remain = info.expire - Utc::now();
                    let color_date = if remain.num_days() > 14 {
                        colored::Color::Green
                    } else if remain.num_days() > 7 {
                        colored::Color::Yellow
                    } else {
                        colored::Color::Red
                    };
                    let format_bytes = |bytes: u64| -> String {
                        HumanBytes(bytes).to_string().color(color_bytes).to_string()
                    };
                    let format_date = |date: DateTime<Utc>| -> String {
                        date.with_timezone(&Local)
                            .format("%F")
                            .to_string()
                            .color(color_date)
                            .to_string()
                    };
                    table.push_record([
                        sub.name.as_str(),
                        format_bytes(info.upload).as_str(),
                        format_bytes(info.download).as_str(),
                        format_bytes(info.total - info.upload - info.download).as_str(),
                        format_date(info.expire).as_str(),
                    ])
                }
            }
            let mut table = table.build();
            table
                .with(Style::rounded())
                .modify(Columns::first(), tabled::settings::Color::FG_BLUE)
                .modify(Columns::new(1..=3), Alignment::right());
            table
        };
        println!("{}", table);
        Ok(())
    }
}
