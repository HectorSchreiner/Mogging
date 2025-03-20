mod config;
mod mogger;

use std::sync::OnceLock;

use config::*;
use mogger::*;
use mogging::*;

pub static MOGGER: OnceLock<Mogger> = OnceLock::new();

fn main() {
    let config = Config::builder()
        .timeformat(Some(TimeFormatType::ClockDateMonthYear))
        .level_format(Some(LevelFormatType::Default))
        .build();
    let mogger = Mogger::new(config, Format::PlainText);
    MOGGER.set(mogger).expect("Logger already initialized");
    debug!("Debug Log");
    info!("Info Log");
    error!("Error Log");
    warn!("Warning Log");
}
