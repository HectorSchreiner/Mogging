mod config;
mod global;
mod mogger;

use global::MOGGER;
use mogger::Mogger;
use mogging::*;

fn main() {
    Mogger::default();

    debug!("Debug Log");
    info!("Info Log");
    error!("Error Log");
    warn!("Warning Log");
}
