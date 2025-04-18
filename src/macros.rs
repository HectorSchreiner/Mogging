///Macro to log an info log, with the global mogger.
///# Example
///```
///info!("Hello, World!);
///```
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
///Macro to log a warning log, with the global mogger.
///# Example
///```
///warn!("Hello, World!);
///```
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
///Macro to log an error log, with the global mogger.
///# Example
///```
///error!("Hello, World!);
///error!(format!("Hello, World!"));
///```
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
///Macro to log a debug log, with the global mogger.
///# Example
///```
///debug!("Hello, World!);
///```
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