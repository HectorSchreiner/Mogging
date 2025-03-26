use mogging::{debug, error, info, warn, LogLevel, Mogger, MOGGER};

#[test]
pub fn initialize_test() {
    let _ = Mogger::default();
    assert!(MOGGER.get().is_some());
}

#[test]
pub fn info_test() {
    let _ = Mogger::default();
    info!("test");
    //assert!(MOGGER.get().is_some());
}

#[test]
pub fn debug_test() {
    let _ = Mogger::default();
    debug!("test");
    //assert!(MOGGER.get().is_some());
}
