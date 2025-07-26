//! behavior_anomaly_test.rs
//! Unit tests for behavior anomaly detection.

#[cfg(test)]
mod tests {
    use kairo::mesh_trust_calculator::TrustScoreCalculator; // Assuming the struct is in this path

    #[test]
    fn test_anomaly_detection_normal_case() {
        let calculator = TrustScoreCalculator::new();
        let baseline = vec![1.0, 2.0, 3.0];
        let current = vec![1.1, 2.2, 3.3]; // Slightly different but similar
        let is_anomaly = calculator.check_behavior_anomaly(&current, &baseline, 0.95);
        assert_eq!(is_anomaly, false);
    }

    #[test]
    fn test_anomaly_detection_abnormal_case() {
        let calculator = TrustScoreCalculator::new();
        let baseline = vec![1.0, 2.0, 3.0];
        let current = vec![-1.0, -2.0, -3.0]; // Opposite direction, low similarity
        let is_anomaly = calculator.check_behavior_anomaly(&current, &baseline, 0.95);
        assert_eq!(is_anomaly, true);
    }
}
