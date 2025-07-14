//! baseline_profile_manager.rs
//! Manages the baseline behavior profiles for each AI agent.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BehaviorProfile {
    pub agent_id: String,
    pub baseline_vector: Vec<f64>,
    pub version: u64,
}

#[derive(Debug)]
pub struct BaselineProfileManager {
    profiles: HashMap<String, BehaviorProfile>,
}

impl BaselineProfileManager {
    pub fn new() -> Self {
        Self { profiles: HashMap::new() }
    }

    pub fn update_profile(&mut self, profile: BehaviorProfile) {
        self.profiles.insert(profile.agent_id.clone(), profile);
    }

    pub fn get_profile(&self, agent_id: &str) -> Option<&BehaviorProfile> {
        self.profiles.get(agent_id)
    }
}
