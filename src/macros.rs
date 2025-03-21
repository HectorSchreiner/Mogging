#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(Level::Info, $msg);
        } else {
            println!("error");
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(Level::Warning, $msg);
        } else {
            println!("error");
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(Level::Error, $msg);
        } else {
            println!("error");
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        if let Some(mogger) = MOGGER.get() {
            mogger.log(Level::Debug, $msg);
        } else {
            println!("error");
        }
    };
}
