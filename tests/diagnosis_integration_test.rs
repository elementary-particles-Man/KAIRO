//! diagnosis_integration_test.rs
//! Unit tests for the integration of behavior diagnosis.

#[cfg(test)]
mod tests {
    use crate::mesh_trust_calculator::TrustScoreCalculator;
    use crate::baseline_profile_manager::{BaselineProfileManager, BehaviorProfile};

    #[test]
    fn test_behavior_verification_with_profile() {
        let calculator = TrustScoreCalculator::new();
        let mut manager = BaselineProfileManager::new();

        // 正常なプロファイルを登録
        let profile = BehaviorProfile {
            agent_id: "agent_001".to_string(),
            baseline_vector: vec![1.0, 2.0, 3.0],
            version: 1,
        };
        manager.update_profile(profile);

        // 正常ケース
        let normal_vector = vec![1.1, 2.1, 2.9];
        let is_anomaly_normal = calculator.verify_agent_behavior(&manager, "agent_001", &normal_vector, 0.95);
        assert_eq!(is_anomaly_normal, false);

        // 異常ケース
        let abnormal_vector = vec![-1.0, -2.0, -3.0];
        let is_anomaly_abnormal = calculator.verify_agent_behavior(&manager, "agent_001", &abnormal_vector, 0.95);
        assert_eq!(is_anomaly_abnormal, true);

        // プロファイルが存在しないケース
        let unknown_vector = vec![1.0, 2.0, 3.0];
        let is_anomaly_unknown = calculator.verify_agent_behavior(&manager, "agent_002", &unknown_vector, 0.95);
        assert_eq!(is_anomaly_unknown, false); // 異常とは判断されない
    }
}
