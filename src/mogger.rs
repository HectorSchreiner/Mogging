#![allow(dead_code)]
use chrono::Utc;
use crossterm::{
    cursor::MoveDown,
    style::{Color, Print, ResetColor, SetForegroundColor},
    *,
};
use std::{fmt::format, io::stdout};

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
        let _ = MOGGER.set(self);
    }

    pub fn new(config: Config, output_format: LogFormat) -> Mogger {
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

        if num >= i32::from(min) && num <= i32::from(max) {
            match self.config.output {
                OutputType::Console => Self::console_write(&self, level, message),
            }
        }
    }

    fn console_write(&self, level: LogLevel, message: &str) {
        // print the level (warn, error...) to the console
        let mut stdout = stdout();

        stdout.execute(Print(format!("{}", message))).unwrap();
        stdout.execute(MoveUp(1)).unwrap();
    }

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
