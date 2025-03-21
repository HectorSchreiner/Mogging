mod config;
mod global;
mod mogger;

use std::sync::OnceLock;

use config::*;
use global::MOGGER;
use mogger::Mogger;
use mogger::*;
use mogging::*;

fn main() {
    Mogger::default().init();

    debug!("Debug Log");
    info!("Info Log");
    error!("Error Log");
    warn!("Warning Log");
}
