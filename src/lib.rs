mod config;
mod logger;
mod macros;

use chrono::Utc;
use std::cell::OnceCell;
use std::sync::Arc;
use std::sync::OnceLock;

use config::Config;

use logger::Logger;

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn logger() {
    println!("Hello, world!");
}
