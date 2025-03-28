#![allow(dead_code)]
use chrono::Utc;
use crossterm::{
    cursor::MoveDown,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
    *,
};
use std::{
    fmt::format,
    io::{stdout, Write},
};

use crate::config::*;
use crate::global::MOGGER;

#[derive(Debug)]
pub struct Mogger {
    pub config: Config,
    pub output_format: LogFormat,
}

impl Mogger {
    // Initializes the mogger, this should be called in all methods that tries to init a mogger
    pub fn init(self) {
        enable_raw_mode().unwrap();
        let _ = MOGGER.set(self);
    }

    pub fn new(config: Config, output_format: LogFormat) -> Mogger {
        let capacity = config.batch_size.clone() as usize;
        Mogger {
            config,
            output_format,
        }
    }

    pub fn create_default_mogger() -> Self {
        let config = Config::builder()
            .set_timeformat(Some(TimeFormatType::ClockDateMonthYear))
            .set_level_format(Some(LevelFormatType::Default))
            .build();

        Mogger::new(config, LogFormat::PlainText)
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let (min, max) = (
            self.config.level_clamp.0.clone(),
            self.config.level_clamp.1.clone(),
        );
        let num = i32::from(level);

        // if level is between the clamp, then match the correct writer.
        if num >= i32::from(min) && num <= i32::from(max) {}
    }

    fn console_write(&self, level: LogLevel, message: &str) {

        // print the level (warn, error...) to the console
        //let mut stdout = stdout();
    }

    fn add_log_to_batch(&self, log: String) {}

    fn get_time(&self) -> String {
        let time = Utc::now();
        let mut formatted = "".to_string();

        if let Some(time_option) = &self.config.time_option {
            match time_option {
                TimeFormatType::Default => formatted = format!("{}", time.format("%H:%M")),
                TimeFormatType::ClockDateMonthYear => {
                    formatted = format!("{}", time.format("%H:%M %d/%m/%Y"))
                }
            }
        }
        format!("{}", formatted)
    }
}

pub fn add_log_to_batch(log: String, mogger: Mogger) {}

#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Undefined,
}

#[derive(Debug)]
pub enum LogFormat {
    PlainText,
}

impl From<i32> for LogLevel {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Debug,
            2 => Self::Info,
            3 => Self::Warning,
            4 => Self::Error,
            _ => Self::Undefined,
        }
    }
}

impl From<LogLevel> for i32 {
    fn from(log_level: LogLevel) -> i32 {
        match log_level {
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warning => 3,
            LogLevel::Error => 4,
            LogLevel::Undefined => 0,
        }
    }
}

// when the mogger is dropped aka. program exited,
// we will disable rawmode
impl Drop for Mogger {
    fn drop(&mut self) {
        stdout().flush().unwrap();
        println!("exited");
        disable_raw_mode().unwrap();
    }
}
