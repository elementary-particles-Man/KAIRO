use crc32fast::Hasher;

use crate::{kairo_p::PAddressRecord, time, witness::Ring, witness::WitnessRecord};

pub struct ClearMini {
    pub ring: Ring,
}

impl ClearMini {
    pub fn new() -> Self {
        time::mono_init();
        Self {
            ring: Ring::new(4096),
        }
    }

    pub fn record(
        &self,
        src: &PAddressRecord,
        dst: &PAddressRecord,
        len: u32,
        flags: u32,
        ip: [u8; 16],
        port: u16,
    ) {
        let mut hasher = Hasher::new();
        hasher.update(&ip);
        let record = WitnessRecord {
            mono: time::now_mono(),
            utc: time::now_utc(),
            src: src.id,
            dst: dst.id,
            len,
            hash32: hasher.finalize(),
            flags,
            port,
            ip,
            ..Default::default()
        };
        self.ring.push(record);
    }
}
