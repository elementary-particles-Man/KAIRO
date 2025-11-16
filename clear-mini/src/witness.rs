use serde::ser::SerializeStruct;
use serde::Serialize;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[repr(C)]
#[derive(Clone)]
pub struct WitnessRecord {
    pub mono: u128,
    pub utc: u128,
    pub src: i32,
    pub dst: i32,
    pub len: u32,
    pub hash32: u32,
    pub flags: u32,
    pub port: u16,
    pub ip: [u8; 16],
    pub pad: [u8; 48],
}

impl Default for WitnessRecord {
    fn default() -> Self {
        Self {
            mono: 0,
            utc: 0,
            src: 0,
            dst: 0,
            len: 0,
            hash32: 0,
            flags: 0,
            port: 0,
            ip: [0; 16],
            pad: [0; 48],
        }
    }
}

impl Serialize for WitnessRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("WitnessRecord", 10)?;
        state.serialize_field("mono", &self.mono)?;
        state.serialize_field("utc", &self.utc)?;
        state.serialize_field("src", &self.src)?;
        state.serialize_field("dst", &self.dst)?;
        state.serialize_field("len", &self.len)?;
        state.serialize_field("hash32", &self.hash32)?;
        state.serialize_field("flags", &self.flags)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("ip", &self.ip)?;
        state.serialize_field("pad", &&self.pad[..])?;
        state.end()
    }
}

pub struct Ring {
    cap: usize,
    buf: Arc<Mutex<VecDeque<WitnessRecord>>>,
}

impl Ring {
    pub fn new(capacity: usize) -> Self {
        Self {
            cap: capacity,
            buf: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
        }
    }

    pub fn push(&self, record: WitnessRecord) {
        let mut guard = self.buf.lock().unwrap();
        if guard.len() == self.cap {
            guard.pop_front();
        }
        guard.push_back(record);
    }

    pub fn snapshot(&self) -> Vec<WitnessRecord> {
        self.buf.lock().unwrap().iter().cloned().collect()
    }
}
