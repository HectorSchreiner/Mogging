#![allow(dead_code)]

use chrono::Utc;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    *,
};
use std::io::stdout;

use crate::config::*;

#[derive(Debug)]
pub struct Mogger {
    pub config: Config,
    pub output_format: Format,
}

impl Mogger {
    pub fn new(config: Config, output_format: Format) -> Self {
        Self {
            config,
            output_format,
        }
    }

    pub fn default() -> Self {
        let config = Config::builder()
            .timeformat(Some(TimeFormatType::ClockDateMonthYear))
            .level_format(Some(LevelFormatType::Default))
            .build();

        Mogger::new(config, Format::PlainText)
    }

    pub fn log(&self, level: Level, message: &str) {
        match self.config.output {
            OutputType::Console => Self::console_write(&self, level, message),
        }
    }

    fn console_write(&self, level: Level, message: &str) {
        self.console_write_level(level);
        self.console_write_time();

        println!("{}", message);
    }

    fn console_write_level(&self, level: Level) {
        match &self.config.level_option {
            Some(_) => {
                let _ = match level {
                    Level::Debug => execute!(stdout(), Print(format!("[{:?}] ", level)),).unwrap(),
                    Level::Info => execute!(
                        stdout(),
                        SetForegroundColor(Color::White),
                        Print(format!("[{:?}] ", level)),
                    )
                    .unwrap(),
                    Level::Warning => execute!(
                        stdout(),
                        SetForegroundColor(Color::Yellow),
                        Print(format!("[{:?}] ", level)),
                    )
                    .unwrap(),
                    Level::Error => execute!(
                        stdout(),
                        SetForegroundColor(Color::Red),
                        Print(format!("[{:?}] ", level)),
                    )
                    .unwrap(),
                };
            }
            None => (), // we might want to do something else if there is a none
        }
        execute!(stdout(), ResetColor).unwrap();
    }

    fn console_write_time(&self) {
        match &self.config.time_option {
            Some(_) => {
                execute!(stdout(), Print(format!("[{:?}] ", Self::get_time(&self)))).unwrap()
            }
            None => (), // we might want to do something else if there is a none
        }
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

#[derive(Debug)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub enum Format {
    PlainText,
}
