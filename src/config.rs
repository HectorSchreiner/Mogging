#[derive(Debug)]
pub struct Config {
    pub output: OutputType,
    pub timeformat: TimeformatType,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub struct ConfigBuilder {
    pub output: OutputType,
    pub timeformat: TimeformatType,
}

impl ConfigBuilder {
    pub fn default() -> ConfigBuilder {
        Self {
            output: OutputType::Console,
            timeformat: TimeformatType::Default,
        }
    }

    pub fn output(mut self, output: OutputType) -> Self {
        self.output = output;
        self
    }

    pub fn timeformat(mut self, format: TimeformatType) -> Self {
        self.timeformat = format;
        self
    }

    pub fn build(self) -> Config {
        Config {
            output: self.output,
            timeformat: self.timeformat,
        }
    }
}
#[derive(Debug)]
pub enum OutputType {
    Console,
}

#[derive(Debug)]
pub enum TimeformatType {
    Default,
    ClockDateMonthYear,
}
