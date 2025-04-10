#![allow(dead_code)]
use chrono::Utc;
use crossterm::terminal::disable_raw_mode;
use std::{
    fmt::Error,
    io::{stdout, BufWriter, Stdout, Write},
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
        let (level_min, level_max) = self.config.level_clamp;
        let [level_min, level_max] = [level_min, level_max].map(usize::from);

        // if level is between the clamp.
        if (level_min..=level_max).contains(&level.into()) {
            self.console_write(level, message);
        }
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
}

#[derive(Debug)]
pub enum LogFormat {
    PlainText,
}

impl TryFrom<usize> for LogLevel {
    type Error = &'static str;
    fn try_from(value: usize) -> Result<Self, &'static str> {
        match value {
            0 => Ok(Self::Debug),
            1 => Ok(Self::Info),
            2 => Ok(Self::Warning),
            3 => Ok(Self::Error),
            _ => Err("Invalid Value!"),
        }
    }
}

impl From<LogLevel> for usize {
    fn from(log_level: LogLevel) -> usize {
        match log_level {
            LogLevel::Debug => 0,
            LogLevel::Info => 1,
            LogLevel::Warning => 2,
            LogLevel::Error => 3,
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
        }
    }
}
// when the mogger is dropped aka. program exited,
// disable the rawdogmode
impl Drop for Mogger {
    fn drop(&mut self) {
        let _ = self.buf_writer.flush();
        disable_raw_mode().unwrap();
    }
}
