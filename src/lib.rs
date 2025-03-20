use std::sync::OnceLock;

use mogger::Mogger;

mod config;
mod mogger;
mod macros;

pub static MOGGER: OnceLock<Mogger> = OnceLock::new();

pub fn logger() {}
