#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        if let Some(logger) = MOGGER.get() {
            logger.log(Level::Info, $msg);
        } else {
            println!("error");
        }
    };
}
#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        if let Some(logger) = MOGGER.get() {
            logger.log(Level::Warning, $msg);
        } else {
            println!("error");
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        if let Some(logger) = MOGGER.get() {
            logger.log(Level::Error, $msg);
        } else {
            println!("error");
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        if let Some(logger) = MOGGER.get() {
            logger.log(Level::Debug, $msg);
        } else {
            println!("error");
        }
    };
}
