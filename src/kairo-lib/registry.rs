use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryEntry {
    pub name: String,
    pub p_address: String,
}

pub fn load_registry(path: &str) -> Result<Vec<RegistryEntry>, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(contents) => {
            let entries: Vec<RegistryEntry> = serde_json::from_str(&contents).unwrap_or_default();
            Ok(entries)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Ok(Vec::new())
            } else {
                Err(e)
            }
        }
    }
}

pub fn save_registry(path: &str, registry: &[RegistryEntry]) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(registry)?;
    fs::write(path, json)
}

pub fn add_entry(path: &str, entry: RegistryEntry) -> Result<(), String> {
    let mut registry = load_registry(path).map_err(|e| e.to_string())?;
    if registry.iter().any(|e| e.name == entry.name) {
        return Err(format!("Agent name '{}' already registered", entry.name));
    }
    if registry.iter().any(|e| e.p_address == entry.p_address) {
        return Err(format!("P address '{}' already registered", entry.p_address));
    }
    registry.push(entry);
    save_registry(path, &registry).map_err(|e| e.to_string())
}
