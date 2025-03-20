#![allow(dead_code)]

#[derive(Debug)]
pub struct Config {
    pub output: OutputType,
    pub time_option: Option<TimeFormatType>,
    pub level_option: Option<LevelFormatType>
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub struct ConfigBuilder {
    pub output: OutputType,
    pub time_option: Option<TimeFormatType>,
    pub level_option: Option<LevelFormatType>
}

impl ConfigBuilder {
    pub fn default() -> ConfigBuilder {
        Self {
            output: OutputType::Console,
            time_option: None,
            level_option: None,
        }
    }

    pub fn level_format(mut self, format: Option<LevelFormatType>) -> Self {
        self.level_option = format;
        self
    }

    pub fn output(mut self, output: OutputType) -> Self {
        self.output = output;
        self
    }

    pub fn timeformat(mut self, format: Option<TimeFormatType>) -> Self {
        self.time_option = format;
        self
    }

    pub fn build(self) -> Config {
        Config {
            output: self.output,
            time_option: self.time_option,
            level_option: self.level_option
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


