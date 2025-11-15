use once_cell::sync::Lazy;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// static mut の代わりに Lazy を使用し、スレッドセーフな初回初期化を保証
static START: Lazy<Instant> = Lazy::new(|| Instant::now());

#[inline]
pub fn init_monotonic_base() {
    // Lazy を使用する場合、明示的な初期化呼び出しは不要。
    // 最初の .elapsed() 呼び出しで自動的に初期化される。
    // 関数自体は、呼び出し元の互換性のために残す（中身は空）。
}

#[inline]
pub fn now_monotonic_ns() -> u128 {
    // unsafe ブロックが不要になり、警告が解消される
    START.elapsed().as_nanos()
}

#[inline]
pub fn now_utc_ns() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default() // パニックを回避（理論上は発生しないがより堅牢に）
        .as_nanos()
}
