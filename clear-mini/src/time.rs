use std::time::{Instant, SystemTime, UNIX_EPOCH};

static mut START: Option<Instant> = None;

pub(crate) fn mono_init() {
    unsafe {
        if START.is_none() {
            START = Some(Instant::now());
        }
    }
}

pub fn now_mono() -> u128 {
    unsafe {
        if START.is_none() {
            START = Some(Instant::now());
        }
        START.unwrap().elapsed().as_nanos()
    }
}

pub fn now_utc() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
