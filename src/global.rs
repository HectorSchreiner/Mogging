use crate::mogger::Mogger;
use std::sync::OnceLock;

pub static MOGGER: OnceLock<Mogger> = OnceLock::new();
