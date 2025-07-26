//! baseline_profile_test.rs
//! Unit tests for the BaselineProfileManager.

#[cfg(test)]
mod tests {
    use kairo::baseline_profile_manager::{BaselineProfileManager, BehaviorProfile};

    #[test]
    fn test_profile_management() {
        let mut manager = BaselineProfileManager::new();
        let profile = BehaviorProfile {
            agent_id: "agent_001".to_string(),
            baseline_vector: vec![1.0, 2.0, 3.0],
            version: 1,
        };
        manager.update_profile(profile.clone());

        let retrieved = manager.get_profile("agent_001").unwrap();
        assert_eq!(retrieved.agent_id, "agent_001");
        assert_eq!(retrieved.version, 1);
    }
}
