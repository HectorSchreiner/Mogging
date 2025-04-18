#![allow(dead_code)]
#[warn(unused_doc_comments)]
use chrono::Utc;
use crossterm::terminal::disable_raw_mode;
use std::{
    io::{stdout, BufWriter, Stdout, Write},
    sync::Mutex,
};

use crate::config::*;
use crate::global::MOGGER;

/// A global static struct that handles the log printing. The Mogger takes a config,
/// an output format and holds a BufWriter.
/// # Examples
/// Create a global Mogger with a default configuration
/// ```no_run
/// Mogger::default();
/// info!("Hello, World!");
/// ```
#[derive(Debug)]
pub struct Mogger {
    pub config: Config,
    pub output_format: LogFormat,
    pub buf_writer: BufWriter<Stdout>,
}

impl Mogger {
    /// Initailizes the Mogger globally.
    pub fn init(self) {
        //enable_raw_mode().unwrap();
        let _ = MOGGER.set(Mutex::new(self));
    }

    /// Create a new mogger, from a configuraiton and an output_format specification.
    /// Returns a Mogger.
    pub fn new<T: Into<Config>>(config: T, output_format: LogFormat) -> Mogger {
        // This mogger still needs initialisation with the init method.
        let config = config.into();
        let buf_writer = BufWriter::with_capacity(4096, stdout()); // why is setting it through the settings slower!!
        Mogger {
            config,
            output_format,
            buf_writer,
        }
    }

    /// Creates and initializes a new Mogger with default options.
    pub fn default() {
        let config = Config::builder()
            .timeformat(TimeFormat::ClockDateMonthYear)
            .level_format(LevelFormat::Default)
            .build();

        Mogger::new(config, LogFormat::PlainText).init();
    }

    /// Outputs the log.
    pub fn log(&mut self, level: LogLevel, message: &str) {
        let (level_min, level_max) = self.config.level_clamp;
        let [level_min, level_max] = [level_min, level_max].map(usize::from);

        // if level is between the clamp.
        if (level_min..=level_max).contains(&level.into()) {
            match self.config.output {
                OutputType::Console => self.console_write(level, message),
                OutputType::File => (),
            }
        }
    }

    /// Writes the log to the console.
    fn console_write(&mut self, level: LogLevel, message: &str) {
        let level_str = level.as_str();

        let Config {
            level_option,
            time_option,
            ..
        } = &self.config;

        match level_option {
            Some(LevelFormat::Default) => write!(self.buf_writer, "[{}]", level_str).unwrap(),
            Some(LevelFormat::Colored) => todo!(),
            None => (),
        }

        if let Some(_time) = time_option {
            write!(self.buf_writer, "[{}] ", self.get_time()).unwrap();
        }

        let _ = write!(self.buf_writer, "{}\n", message);
        self.buf_writer.flush().unwrap();
    }

    fn get_time(&self) -> String {
        let now = Utc::now();

        match &self.config.time_option {
            Some(TimeFormat::Default) => now.format("%H:%M").to_string(),
            Some(TimeFormat::ClockDateMonthYear) => now.format("%H:%M %d/%m/%Y").to_string(),
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
        self.buf_writer.flush().unwrap();
        println!("exited program, mogger dripped");
        disable_raw_mode().unwrap();
    }
}
