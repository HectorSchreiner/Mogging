use crate::config::{self, *};
use chrono::Utc;

#[derive(Debug)]
pub struct Logger {
    pub config: Config,
    pub format: Format,
}

impl Logger {
    pub fn new(config: Config, format: Format) -> Self {
        Self { config, format }
    }

    pub fn log(&self, level: Level, message: String) {
        match self.config.output {
            OutputType::Console => Self::console_write(level, message),
        }
    }

    fn console_write(level: Level, message: String) {
        // seperated like this, because i want to use colors in the future
        println!(
            "[Level: {:?} Timestamp: {}], message {}",
            level,
            Utc::now(),
            message
        );
    }
}

#[derive(Debug)]
pub enum Level {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub enum Format {
    PlainText,
}
