use mogging::{debug, error, info, warn, LogLevel, Mogger, MOGGER};

#[test]
pub fn initialize_test() {
    let _ = Mogger::default();
    assert!(MOGGER.get().is_some());
    info_test();
    debug_test();
}

pub fn info_test() {
    info!("asdkaljds");
    info!("something");
    assert!(MOGGER.get().is_some());
}

pub fn debug_test() {
    assert!(MOGGER.get().is_some());
}
