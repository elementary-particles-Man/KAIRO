//! Loads WAU thresholds from YAML.
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Hysteresis {
    pub up_margin: f32,
    pub down_margin: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WauThresholds {
    pub personal: f32,
    pub family: f32,
    pub group: f32,
    pub community: f32,
    pub world: f32,
    pub hysteresis: Hysteresis,
}

impl WauThresholds {
    pub fn load_from(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let s = std::fs::read_to_string(path)?;
        let v: Self = serde_yaml::from_str(&s)?;
        Ok(v)
    }
}
