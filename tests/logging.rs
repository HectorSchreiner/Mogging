use mogging::{LogLevel, Mogger, MOGGER};

#[test]
pub fn initialize_test() {
    Mogger::default();
    assert!(MOGGER.get().is_some());
}

#[test]
pub fn log_to_i32_conversion_test() {
    assert_eq!(usize::from(LogLevel::Debug), 0);
    assert_eq!(LogLevel::Debug, LogLevel::try_from(0).unwrap());
}
