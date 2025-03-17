mod config;
mod logger;

use std::fmt::Formatter;
use std::sync::OnceLock;

use chrono::format;
use config::*;
use logger::*;

static LOGGER: OnceLock<Logger> = OnceLock::new();

fn main() {
    let config = Config::builder().build();
    let logger = Logger::new(config, Format::PlainText);
    LOGGER.set(logger).expect("Logger already initialized");
    println!("main");
}
