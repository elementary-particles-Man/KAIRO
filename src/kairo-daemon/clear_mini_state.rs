use clear_mini::api::ClearMini;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub(crate) static CLEAR_MINI: Lazy<Mutex<ClearMini>> = Lazy::new(|| Mutex::new(ClearMini::new()));
