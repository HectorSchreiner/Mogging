mod config;
mod logger;

use std::fmt::Formatter;
use std::sync::OnceLock;

use chrono::format;
use config::*;
use logger::*;
use loggy::{error, info, warn};

static LOGGER: OnceLock<Logger> = OnceLock::new();

fn main() {
    let mut config = Config::builder()
        .timeformat(TimeformatType::ClockDateMonthYear)
        .build();
    let logger = Logger::new(config, Format::PlainText);
    LOGGER.set(logger).expect("Logger already initialized");

    info!("message");

    error!("error log");

    warn!("Warning log");
}
