mod config;
mod global;
mod macros;
mod mogger;

use global::MOGGER;
use mogger::Mogger;
use mogger::*;

fn main() {
    Mogger::default();

    debug!("Debug Log");
    info!("Info Log");
    error!("Error Log");
    warn!("Warning Log");
}
