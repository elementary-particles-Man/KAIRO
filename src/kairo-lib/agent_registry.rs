use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryEntry {
    pub public_key: String,
    pub p_address: String,
    #[serde(default)]
    pub registered_at: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AgentRegistry {
    pub entries: Vec<RegistryEntry>,
}

impl AgentRegistry {
    pub fn load(path: &str) -> io::Result<Self> {
        let file = File::open(path).or_else(|_| File::create(path))?;
        let reader = BufReader::new(file);
        let entries = serde_json::from_reader(reader).unwrap_or_default();
        Ok(Self { entries })
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.entries)?;
        Ok(())
    }

    pub fn validate(&self, p_address: &str, public_key: &str) -> bool {
        self.entries.iter().any(|e| e.p_address == p_address && e.public_key == public_key)
    }
}
