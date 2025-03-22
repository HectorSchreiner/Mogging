use mogging::{info, LogLevel, Mogger, MOGGER};

#[test]
pub fn initialize() {
    Mogger::default();
    assert!(MOGGER.get().is_some());
}

#[test]
pub fn info_macro_test() {
    Mogger::default();
    info!("This should run");
}
