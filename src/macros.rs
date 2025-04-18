///Macro to log a log with a specified LogLevel, with the global mogger.
///# Example
///```no_run
///log(Mogger::LogLevel::Error, "Hello mr. worldwide!");
///```
#[macro_export]
macro_rules! log {
    ($level:expr, $($msg:tt)*) => {
        let level: $crate::LogLevel = $level;

        if let Some(mogger_mutex) = $crate::MOGGER.get() {
            let mut mogger = mogger_mutex.lock().unwrap();
            mogger.log(level, format_args!($($msg)*));
        } else {
            panic!("Panicked when trying to print info log: MOGGER is not initialized.");
        }
    };
}
///Macro to log an info log, with the global mogger.
///# Example
///```no_run
///info!("Hello, World!);
///info!("{}", arg);
///```
#[macro_export]
macro_rules! info {
    ($($msg:tt)*) => {
        log!($crate::LogLevel::Info, $($msg)*);
    };
}
///Macro to log a warning log, with the global mogger.
///# Example
///```no_run
///warn!("Hello, World!);
///warn!("{}", arg);
///```
#[macro_export]
macro_rules! warn {
    ($($msg:tt)*) => {
        log!($crate::LogLevel::Warn, $($msg)*);
    };
}
///Macro to log an error log, with the global mogger.
///# Example
///```no_run
///error!("Hello, World!");
///error!("{}", arg);
///```
#[macro_export]
macro_rules! error {
    ($($msg:tt)*) => {
        log!($crate::LogLevel::Error, $($msg)*);
    };
}
///Macro to log a debug log, with the global mogger.
///# Example
///```no_run
///debug!("Hello, World!);
///```
#[macro_export]
macro_rules! debug {
    ($($msg:tt)*) => {
        log!($crate::LogLevel::Debug, $($msg)*);
    };
}
