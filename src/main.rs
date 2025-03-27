mod config;
mod global;
mod macros;
mod mogger;

use std::fmt::format;
use std::io::{stdout, Write};

use config::Config;
use crossterm::style::Print;
use crossterm::{execute, queue};
use global::MOGGER;
use mogger::Mogger;
use mogger::*;

fn main() {
    let config = Config::builder().build();

    Mogger::new(config, LogFormat::PlainText).init();

    debug!("Debug Log");
    info!("Info Log");
    warn!("Warning Log");
    error!("Error Log");
}
