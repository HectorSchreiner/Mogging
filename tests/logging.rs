use mogging::{debug, error, info, warn, LogLevel, Mogger, MOGGER};

#[test]
pub fn initialize_test() {
    Mogger::default();
    assert!(MOGGER.get().is_some());
}

#[test]
pub fn info_test() {
    info!("Debug Log");
}

#[test]
pub fn debug_test() {
    debug!("Debug Log");
}
