#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(LogLevel::Info, $msg);
        } else {
            panic!("Paniced when trying to print info log, make sure the MOGGER is initialized")
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(LogLevel::Warning, $msg);
        } else {
            panic!("Paniced when trying to print info log, make sure the MOGGER is initialized")
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(LogLevel::Error, $msg);
        } else {
            panic!("Paniced when trying to print info log, make sure the MOGGER is initialized")
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(LogLevel::Debug, $msg);
        } else {
            panic!("Paniced when trying to print info log, make sure the MOGGER is initialized")
        }
    };
}
