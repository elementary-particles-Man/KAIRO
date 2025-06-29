use chrono::{Duration, Utc};
use rust_core::LogRecorder;

#[test]
fn rotates_key_after_24_hours() {
    let mut logger = LogRecorder::new();
    let first_key = logger.key().clone();
    logger.set_key_start(Utc::now() - Duration::hours(25));
    // call sign to trigger rotation
    logger.sign(b"test");
    assert_ne!(logger.key(), &first_key);
    assert!(logger.key_start() > Utc::now() - Duration::minutes(1));
}
