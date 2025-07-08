// D:\dev\KAIRO\rust-core\src\log_recorder.rs
use hmac::Hmac;
use sha2::Sha256;
use chrono::{DateTime, Utc};

type HmacSha256 = Hmac<Sha256>;

pub struct LogRecorder {
    key: Vec<u8>,
    key_creation_time: DateTime<Utc>,
}

impl LogRecorder {
    pub fn new(initial_key: Vec<u8>) -> Self {
        Self {
            key: initial_key,
            key_creation_time: Utc::now(),
        }
    }
    // ダミーのキーローテーション関数
    pub fn rotate_key_if_needed(&mut self) {
        self.key = vec![0; 32]; // 新しいキーに更新
        self.key_creation_time = Utc::now();
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }
}

#[cfg(test)]
mod tests {
    use super::LogRecorder;
    use chrono::Utc;

    #[test]
    fn test_log_recorder_new() {
        let recorder = LogRecorder::new(vec![0; 32]);
        assert_eq!(recorder.key.len(), 32);
    }

    #[test]
    fn test_log_recorder_rotate_key_if_needed() {
        let mut recorder = LogRecorder::new(vec![0; 32]);
        let initial_key = recorder.key.clone();
        let initial_time = Utc::now();

        std::thread::sleep(std::time::Duration::from_millis(10));

        recorder.rotate_key_if_needed();

        assert_eq!(recorder.key, vec![0; 32]); 
        assert!(recorder.key_creation_time > initial_time);
    }
}



