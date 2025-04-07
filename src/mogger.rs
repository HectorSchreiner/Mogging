#![allow(dead_code)]
use chrono::Utc;
use crossterm::terminal::disable_raw_mode;
use std::{
    io::{stdout, BufWriter, Stdout, StdoutLock, Write},
    sync::Mutex,
};

use crate::config::*;
use crate::global::MOGGER;

#[derive(Debug)]
pub struct Mogger {
    pub config: Config,
    pub output_format: LogFormat,
    pub buf_writer: BufWriter<Stdout>,
}

impl Mogger {
    // Initializes the mogger, this should be called in all methods that tries to init a mogger
    pub fn init(self) {
        //enable_raw_mode().unwrap();
        let _ = MOGGER.set(Mutex::new(self));
    }

    pub fn new<T: Into<Config>>(config: T, output_format: LogFormat) -> Mogger {
        let config = config.into();
        let batch_size = 4096;
        let buf_writer = BufWriter::with_capacity(batch_size, stdout()); // why is setting it through the settings slower!!
        Mogger {
            config,
            output_format,
            buf_writer,
        }
    }

    pub fn create_default_mogger() -> Self {
        let config = Config::builder()
            .timeformat(Some(TimeFormatType::ClockDateMonthYear))
            .level_format(Some(LevelFormatType::Default))
            .batch_size(4096)
            .build();

        Mogger::new(config, LogFormat::PlainText)
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        // if level is between the clamp, then match the correct writer.
        // mangler lidt
        self.console_write(level, message);
    }

    fn console_write(&mut self, level: LogLevel, message: &str) {
        let time = self.get_time_inline(); // Inlined + minimal alloc
        let level_str = level.as_str();

        let _ = write!(self.buf_writer, "[{}][{}] {}\n", level_str, time, message);
    }

    fn get_time_inline(&self) -> String {
        let now = Utc::now();

        match &self.config.time_option {
            Some(TimeFormatType::Default) => now.format("%H:%M").to_string(),
            Some(TimeFormatType::ClockDateMonthYear) => now.format("%H:%M %d/%m/%Y").to_string(),
            _ => String::new(),
        }
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
impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warning => "Warning",
            LogLevel::Error => "Error",
            LogLevel::Undefined => "Undefined",
        }
    }
}
// when the mogger is dropped aka. program exited,
// we will disable rawmode
impl Drop for Mogger {
    fn drop(&mut self) {
        let _ = self.buf_writer.flush();
        disable_raw_mode().unwrap();
    }
}
