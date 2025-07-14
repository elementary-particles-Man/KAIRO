/// Simple trust score calculator skeleton.
/// Additional trust computation logic can be implemented as needed.
pub struct TrustScoreCalculator;

impl TrustScoreCalculator {
    /// Create a new calculator instance.
    pub fn new() -> Self {
        Self
    }

    pub fn check_behavior_anomaly(&self, current_vector: &[f64], baseline_vector: &[f64], cosine_threshold: f64) -> bool {
        let similarity = self.cosine_similarity(current_vector, baseline_vector);

        // 類似度が指定した閾値を下回った場合に異常と判断
        if similarity < cosine_threshold {
            println!("Behavior anomaly detected: Cosine Similarity {} is below threshold {}", similarity, cosine_threshold);
            return true;
        }
        false
    }

    fn cosine_similarity(&self, vec1: &[f64], vec2: &[f64]) -> f64 {
        let dot_product = vec1.iter().zip(vec2).map(|(a, b)| a * b).sum::<f64>();
        let norm_a = vec1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
        let norm_b = vec2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }
}
