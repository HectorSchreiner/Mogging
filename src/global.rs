use crate::mogger::Mogger;
use std::sync::{Mutex, OnceLock};

pub static MOGGER: OnceLock<Mutex<Mogger>> = OnceLock::new();
