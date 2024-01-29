use chrono::{DateTime, Local, Utc};
use clap::ValueEnum;
use colored::{Color, Colorize};
use indicatif::HumanBytes;
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::Table;

use crate::sub::Sub;

pub trait FmtTable {
    fn fmt_table(&self, fields: &[Field]) -> Table;
}

pub trait FmtRow {
    fn fmt_row(&self, fields: &[Field]) -> Vec<String> {
        fields
            .iter()
            .map(|field| match field {
                Field::Name => self.fmt_name(),
                Field::Url => self.fmt_url(),
                Field::Download => self.fmt_download(),
                Field::Upload => self.fmt_upload(),
                Field::Total => self.fmt_total(),
                Field::Traffic => self.fmt_traffic(),
                Field::Expire => self.fmt_expire(),
            })
            .collect()
    }

    fn fmt_name(&self) -> String {
        match self.name() {
            Some(name) => match self.url() {
                Some(_) => name.bold().blue().to_string(),
                None => name.dimmed().to_string(),
            },
            None => String::new(),
        }
    }

    fn fmt_url(&self) -> String {
        match self.url() {
            Some(url) => url.bold().blue().to_string(),
            None => String::new(),
        }
    }

    fn fmt_download(&self) -> String {
        match self.download() {
            Some(download) => HumanBytes(download)
                .to_string()
                .bold()
                .color(self.color_traffic())
                .to_string(),
            None => String::new(),
        }
    }

    fn fmt_upload(&self) -> String {
        match self.upload() {
            Some(upload) => HumanBytes(upload)
                .to_string()
                .bold()
                .color(self.color_traffic())
                .to_string(),
            None => String::new(),
        }
    }

    fn fmt_total(&self) -> String {
        match self.total() {
            Some(total) => HumanBytes(total)
                .to_string()
                .bold()
                .color(self.color_traffic())
                .to_string(),
            None => String::new(),
        }
    }

    fn fmt_traffic(&self) -> String {
        match (self.download(), self.upload(), self.total()) {
            (Some(download), Some(upload), Some(total)) => format!(
                "{} + {} = {} / {}",
                HumanBytes(download),
                HumanBytes(upload),
                HumanBytes(download + upload),
                HumanBytes(total)
            )
            .bold()
            .color(self.color_traffic())
            .to_string(),
            _ => String::new(),
        }
    }

    fn fmt_expire(&self) -> String {
        match self.expire() {
            Some(expire) => expire
                .with_timezone(&Local)
                .format("%F %T")
                .to_string()
                .bold()
                .color(self.color_date())
                .to_string(),
            None => String::new(),
        }
    }

    fn color_traffic(&self) -> Color {
        let download = match self.download() {
            Some(download) => download,
            None => return Color::White,
        };
        let upload = match self.upload() {
            Some(upload) => upload,
            None => return Color::White,
        };
        let total = match self.total() {
            Some(total) => total,
            None => return Color::White,
        };
        let ratio = (download + upload) as f64 / (total as f64);
        if ratio < 0.6 {
            Color::Green
        } else if ratio < 0.8 {
            Color::Yellow
        } else {
            Color::Red
        }
    }

    fn color_date(&self) -> Color {
        let expire = match self.expire() {
            Some(expire) => expire,
            None => return Color::White,
        };
        let now = Utc::now();
        let days = (expire - now).num_days();
        if days < 7 {
            Color::Red
        } else if days < 28 {
            Color::Yellow
        } else {
            Color::Green
        }
    }

    fn name(&self) -> Option<&str>;

    fn url(&self) -> Option<&str>;

    fn download(&self) -> Option<u64>;

    fn upload(&self) -> Option<u64>;

    fn total(&self) -> Option<u64>;

    fn expire(&self) -> Option<DateTime<Utc>>;
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Field {
    Name,
    Url,
    Download,
    Upload,
    Total,
    Traffic,
    Expire,
}

impl Field {
    fn as_str(&self) -> &str {
        match self {
            Field::Name => "Name",
            Field::Url => "URL",
            Field::Download => "Download",
            Field::Upload => "Upload",
            Field::Total => "Total",
            Field::Traffic => "Traffic",
            Field::Expire => "Expire",
        }
    }
}

impl<R> FmtTable for Vec<R>
where
    R: FmtRow,
{
    fn fmt_table(&self, fields: &[Field]) -> Table {
        let mut builder = Builder::new();
        builder.push_record(fields.iter().map(|field| field.as_str()));
        self.iter().map(|row| row.fmt_row(fields)).for_each(|row| {
            builder.push_record(row);
        });
        let mut table = builder.build();
        table.with(Style::rounded());
        table
    }
}

impl FmtRow for Sub {
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn download(&self) -> Option<u64> {
        Some(self.user_info.as_ref()?.download)
    }

    fn upload(&self) -> Option<u64> {
        Some(self.user_info.as_ref()?.upload)
    }

    fn total(&self) -> Option<u64> {
        Some(self.user_info.as_ref()?.total)
    }

    fn expire(&self) -> Option<DateTime<Utc>> {
        Some(self.user_info.as_ref()?.expire)
    }
}
