use mogging::{LogLevel, Mogger, MOGGER};

#[test]
pub fn initialize_test() {
    Mogger::create_default_mogger().init();
    assert!(MOGGER.get().is_some());
}

#[test]
pub fn log_to_i32_conversion_test() {
    assert_eq!(i32::from(LogLevel::Debug), 1);
    assert_eq!(LogLevel::Debug, LogLevel::from(1));
}
