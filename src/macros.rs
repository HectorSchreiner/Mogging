use crate::config::*;
use crate::logger::*;

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        if let Some(logger) = LOGGER.get() {
            logger.log(Level::Info, $msg);
        } else {
            println!("error");
        }
    };
}
#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        if let Some(logger) = LOGGER.get() {
            logger.log(Level::Warning, $msg);
        } else {
            println!("error");
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        if let Some(logger) = LOGGER.get() {
            logger.log(Level::Error, $msg);
        } else {
            println!("error");
        }
    };
}
