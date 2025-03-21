mod config;
mod mogger;

use std::sync::OnceLock;

use config::*;
use mogger::*;
use mogging::*;

pub static MOGGER: OnceLock<Mogger> = OnceLock::new();

fn main() {
    let mogger = Mogger::default();
    MOGGER.set(mogger).expect("Logger already initialized");

    debug!("Debug Log");
    info!("Info Log");
    error!("Error Log");
    warn!("Warning Log");
}
