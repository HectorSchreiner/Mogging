#![allow(dead_code)]

use std::io::stdout;
use chrono::Utc;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    *,
};

use crate::config::*;

#[derive(Debug)]
pub struct Mogger {
    pub config: Config,
    pub format: Format,
}

impl Mogger {
    pub fn new(config: Config, format: Format) -> Self {
        Self { config, format }
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
                    Level::Debug => execute!(stdout(), Print(format!("[{:?}] ", level)), SetForegroundColor(Color::White),).unwrap(),
                    Level::Info => execute!(stdout(), Print(format!("[{:?}] ", level)), SetForegroundColor(Color::White),).unwrap(),
                    Level::Warning => execute!(stdout(), Print(format!("[{:?}] ", level)), SetForegroundColor(Color::Yellow),).unwrap(),
                    Level::Error => execute!(stdout(), Print(format!("[{:?}] ", level)), SetForegroundColor(Color::Red),).unwrap(),
                };
                execute!(stdout(), ResetColor).unwrap();
            },
            None => (), // we might want to do something else if there is a none
        }
    }

    fn console_write_time(&self) {
        match &self.config.time_option {
            Some(_) => execute!(stdout(), Print(format!("[{:?}] ", Self::get_time(&self)))).unwrap(),
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
