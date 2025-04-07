mod config;
mod global;
mod macros;
mod mogger;

pub use config::*;
pub use global::MOGGER;
pub use mogger::LogFormat;
pub use mogger::LogLevel;
pub use mogger::Mogger;

pub fn logger() {}
