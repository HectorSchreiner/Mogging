use std::io::{stdout, Write};

use crate::config::{self, *};
use chrono::{DateTime, Utc};
use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, SetStyle},
    *,
};

#[derive(Debug)]
pub struct Logger {
    pub config: Config,
    pub format: Format,
}

impl Logger {
    pub fn new(config: Config, format: Format) -> Self {
        Self { config, format }
    }

    pub fn log(&self, level: Level, message: &str) {
        match self.config.output {
            OutputType::Console => Self::console_write(&self, level, message),
        }
    }

    fn console_write(&self, level: Level, message: &str) {
        // seperated like this, because i want to use colors in the future
        //
        match level {
            Level::Info => stdout().execute(SetForegroundColor(Color::White)),
            Level::Warning => stdout().execute(SetForegroundColor(Color::Yellow)),
            Level::Error => stdout().execute(SetForegroundColor(Color::Red)),
        };

        execute!(stdout(), Print(format!("[{:?}]", level)), ResetColor,).unwrap();
        println!("[Time: {}] Message: {}", Self::get_time(&self), message);
    }

    fn get_time(&self) -> String {
        let mut time = Utc::now();
        let mut formatted = "".to_string();

        match self.config.timeformat {
            TimeformatType::Default => formatted = format!("{}", time.format("%H:%M")),
            TimeformatType::ClockDateMonthYear => {
                formatted = format!("{}", time.format("%H:%M %d/%m/%Y"))
            }
        }
        format!("{}", formatted)
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
