use chrono::{DateTime, Utc};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryEntry {
    pub name: String,
    pub p_address: String,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub last_contact: Option<DateTime<Utc>>,
}

pub fn load_registry(path: &str) -> Result<Vec<RegistryEntry>, std::io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    FileExt::lock_shared(&file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    FileExt::unlock(&file)?;
    if contents.trim().is_empty() {
        Ok(Vec::new())
    } else {
        let entries: Vec<RegistryEntry> = serde_json::from_str(&contents).unwrap_or_default();
        Ok(entries)
    }
}

pub fn save_registry(path: &str, registry: &[RegistryEntry]) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.lock_exclusive()?;
    let json = serde_json::to_string_pretty(registry)?;
    file.write_all(json.as_bytes())?;
    FileExt::unlock(&file)?;
    Ok(())
}

pub fn register_agent(path: &str, entry: RegistryEntry) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.lock_exclusive().map_err(|e| e.to_string())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    let mut registry: Vec<RegistryEntry> = if contents.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).unwrap_or_default()
    };

    if registry.iter().any(|e| !e.deleted && e.name == entry.name) {
        FileExt::unlock(&file).ok();
        return Err(format!("Agent name '{}' already registered", entry.name));
    }
    if registry
        .iter()
        .any(|e| !e.deleted && e.p_address == entry.p_address)
    {
        FileExt::unlock(&file).ok();
        return Err(format!(
            "P address '{}' already registered",
            entry.p_address
        ));
    }

    registry.push(entry);

    file.set_len(0).map_err(|e| e.to_string())?;
    file.seek(std::io::SeekFrom::Start(0))
        .map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&registry).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    FileExt::unlock(&file).ok();
    Ok(())
}

pub fn soft_delete_agent(path: &str, name: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    file.lock_exclusive().map_err(|e| e.to_string())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    let mut registry: Vec<RegistryEntry> = serde_json::from_str(&contents).unwrap_or_default();

    match registry.iter_mut().find(|e| e.name == name && !e.deleted) {
        Some(entry) => entry.deleted = true,
        None => {
            FileExt::unlock(&file).ok();
            return Err(format!("Agent '{}' not found", name));
        }
    }

    file.set_len(0).map_err(|e| e.to_string())?;
    file.seek(std::io::SeekFrom::Start(0))
        .map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&registry).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    FileExt::unlock(&file).ok();
    Ok(())
}

pub fn add_entry(path: &str, entry: RegistryEntry) -> Result<(), String> {
    register_agent(path, entry)
}
