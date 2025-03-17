mod config;
mod logger;
mod macros;

use chrono::Utc;
use macros::*;
use std::cell::OnceCell;
use std::sync::Arc;
use std::sync::OnceLock;

use config::Config;

use logger::Logger;

pub fn logger() {
    println!("Hello, world!");
}
