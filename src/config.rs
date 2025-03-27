#![allow(dead_code)]

use std::error::Error;

use crate::LogLevel;

#[derive(Debug)]
pub struct Config {
    pub output: OutputType,
    pub time_option: Option<TimeFormatType>,
    pub level_option: Option<LevelFormatType>,
    pub batch_size: i32,
    pub flush_timer: f32,
    pub level_clamp: Option<(LogLevel, LogLevel)>,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub struct ConfigBuilder {
    pub output: OutputType,
    pub time_option: Option<TimeFormatType>,
    pub level_option: Option<LevelFormatType>,
    pub batch_size: i32,
    pub flush_timer: f32,
    pub level_clamp: Option<(LogLevel, LogLevel)>,
}

impl ConfigBuilder {
    pub fn default() -> ConfigBuilder {
        Self {
            output: OutputType::Console,
            time_option: None,
            level_option: None,
            batch_size: 0,
            flush_timer: 0.0,
            level_clamp: None,
        }
    }

    pub fn set_output(mut self, output: OutputType) -> Self {
        self.output = output;
        self
    }

    pub fn set_timeformat(mut self, format: Option<TimeFormatType>) -> Self {
        self.time_option = format;
        self
    }

    pub fn set_level_format(mut self, format: Option<LevelFormatType>) -> Self {
        self.level_option = format;
        self
    }

    pub fn set_batch_size(mut self, batch_size: i32) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn set_flush_timer(mut self, flush_timer: f32) -> Self {
        self.flush_timer = flush_timer;
        self
    }

    pub fn build(self) -> Config {
        Config {
            output: self.output,
            time_option: self.time_option,
            level_option: self.level_option,
            batch_size: self.batch_size,
            flush_timer: self.flush_timer,
            level_clamp: self.level_clamp,
        }
    }
}
#[derive(Debug)]
pub enum OutputType {
    Console,
}

#[derive(Debug)]
pub enum TimeFormatType {
    Default,
    ClockDateMonthYear,
}

#[derive(Debug)]
pub enum LevelFormatType {
    Default,
}
