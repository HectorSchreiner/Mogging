#![allow(dead_code)]
use crate::LogLevel;

#[derive(Debug)]
pub struct Config {
    pub output: OutputType,
    pub time_option: Option<TimeFormatType>,
    pub level_option: Option<LevelFormatType>,
    pub batch_size: usize,
    pub flush_timer: f32,
    pub level_clamp: (LogLevel, LogLevel),
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl From<ConfigBuilder> for Config {
    fn from(value: ConfigBuilder) -> Self {
        value.build()
    }
}

pub struct ConfigBuilder {
    pub output: OutputType,
    pub time_option: Option<TimeFormatType>,
    pub level_option: Option<LevelFormatType>,
    pub batch_size: usize,
    pub flush_timer: f32,
    // min, max
    pub level_clamp: (LogLevel, LogLevel),
}

impl ConfigBuilder {
    pub fn default() -> ConfigBuilder {
        Self {
            output: OutputType::Console,
            time_option: Some(TimeFormatType::Default),
            level_option: Some(LevelFormatType::Default),
            batch_size: 0,
            flush_timer: 0.0,
            level_clamp: (LogLevel::Debug, LogLevel::Error),
        }
    }

    pub fn output(mut self, output: OutputType) -> Self {
        self.output = output;
        self
    }

    pub fn timeformat(mut self, format: Option<TimeFormatType>) -> Self {
        self.time_option = format;
        self
    }

    pub fn level_format(mut self, format: Option<LevelFormatType>) -> Self {
        self.level_option = format;
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn flush_timer(mut self, flush_timer: f32) -> Self {
        self.flush_timer = flush_timer;
        self
    }

    pub fn level_clamp(mut self, level_clamp: (LogLevel, LogLevel)) -> Self {
        self.level_clamp = level_clamp;
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
