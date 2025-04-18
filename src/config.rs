#![allow(dead_code)]
use crate::LogLevel;

#[derive(Debug)]
pub struct Config {
    pub output: OutputType,
    pub time_option: Option<TimeFormat>,
    pub level_option: Option<LevelFormat>,
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
    pub time_option: Option<TimeFormat>,
    pub level_option: Option<LevelFormat>,
    // min, max
    pub level_clamp: (LogLevel, LogLevel),
}

impl ConfigBuilder {
    pub fn default() -> ConfigBuilder {
        Self {
            output: OutputType::Console,
            time_option: None,
            level_option: None,
            level_clamp: (LogLevel::Debug, LogLevel::Error),
        }
    }

    pub fn output(mut self, output: OutputType) -> Self {
        self.output = output;
        self
    }

    pub fn timeformat(mut self, format: TimeFormat) -> Self {
        self.time_option = Some(format);
        self
    }

    pub fn level_format(mut self, format: LevelFormat) -> Self {
        self.level_option = Some(format);
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
            level_clamp: self.level_clamp,
        }
    }
}
#[derive(Debug)]
pub enum OutputType {
    Console,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    Default,
    ClockDateMonthYear,
}

#[derive(Debug)]
pub enum LevelFormat {
    Default,
    Colored,
}
