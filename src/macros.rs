#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        if let Some(mogger_mutex) = $crate::MOGGER.get() {
            let mut mogger = mogger_mutex.lock().unwrap();
            mogger.log($crate::LogLevel::Info, $msg);
        } else {
            panic!("Panicked when trying to print info log: MOGGER is not initialized.");
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        if let Some(mogger_mutex) = $crate::MOGGER.get() {
            let mut mogger = mogger_mutex.lock().unwrap();
            mogger.log($crate::LogLevel::Warning, $msg);
        } else {
            panic!("Panicked when trying to print warning log: MOGGER is not initialized.");
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        if let Some(mogger_mutex) = $crate::MOGGER.get() {
            let mut mogger = mogger_mutex.lock().unwrap();
            mogger.log($crate::LogLevel::Error, $msg);
        } else {
            panic!("Panicked when trying to print error log: MOGGER is not initialized.");
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        if let Some(mogger_mutex) = $crate::MOGGER.get() {
            let mut mogger = mogger_mutex.lock().unwrap();
            mogger.log($crate::LogLevel::Debug, $msg);
        } else {
            panic!("Panicked when trying to print debug log: MOGGER is not initialized.");
        }
    };
}
