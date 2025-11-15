#[repr(C)]
#[derive(Clone, Copy)]
pub struct PAddressRecord {
    pub id: i32,
    pub nickname: [u8; 60],
}

impl Default for PAddressRecord {
    fn default() -> Self {
        Self {
            id: 0,
            nickname: [0; 60],
        }
    }
}

impl PAddressRecord {
    pub fn new(id: i32, nickname: &str) -> Self {
        let mut record = Self::default();
        let mut used = 0usize;
        for c in nickname.chars() {
            let mut buf = [0; 4];
            let encoded = c.encode_utf8(&mut buf).as_bytes();
            if used + encoded.len() > 60 {
                break;
            }
            record.nickname[used..used + encoded.len()].copy_from_slice(encoded);
            used += encoded.len();
        }
        record.id = id;
        record
    }
}
