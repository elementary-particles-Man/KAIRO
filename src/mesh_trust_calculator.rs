//! mesh_trust_calculator.rs
//! Implements Peer Review / Gossip based distributed trust score calculation.
//! Handles WAU (Who Are You) authentication and Sybil attack resistance.

use crate::baseline_profile_manager::{BaselineProfileManager, BehaviorProfile};

// Temporarily define Scope here to avoid circular dependency in initial generation
// In actual implementation, Scope will be imported from mesh_scope_manager.rs
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scope {
    Personal = 0,
    Family,
    Group,
    Community,
    World,
}

pub struct TrustScoreCalculator {}

impl TrustScoreCalculator {
    pub fn new() -> Self { Self {} }

    pub fn calculate_trust_score(
        self_trust: f64,
        peer_scores: &[f64],
        gossip_agreement: f64,
        scope: Scope,
    ) -> f64 {
        let weight_self = 0.4;
        let weight_peer = 0.4;
        let weight_gossip = 0.2;

        let peer_avg: f64 = if peer_scores.is_empty() {
            0.0
        } else {
            peer_scores.iter().sum::<f64>() / peer_scores.len() as f64
        };

        let mut trust_score = (weight_self * self_trust)
                            + (weight_peer * peer_avg)
                            + (weight_gossip * gossip_agreement);

        // Sybil attack resistance: Halve trust if insufficient peer reviews
        let min_peer_reviews = match scope {
            Scope::Personal => 1,
            Scope::Family => 3,
            _ => 5,
        };

        if peer_scores.len() < min_peer_reviews {
            trust_score *= 0.5;
        }

        trust_score.clamp(0.0, 1.0)
    }

    pub fn verify_wa_u(trust_score: f64, scope: Scope) -> bool {
        let required_threshold = match scope {
            Scope::Personal => 0.25,
            Scope::Family => 0.50,
            Scope::Group => 0.75,
            Scope::Community => 0.90,
            Scope::World => 0.99,
        };
        trust_score >= required_threshold
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

    pub fn verify_agent_behavior(
        &self,
        profile_manager: &BaselineProfileManager,
        agent_id: &str,
        current_vector: &[f64],
        cosine_threshold: f64
    ) -> bool {
        if let Some(profile) = profile_manager.get_profile(agent_id) {
            return self.check_behavior_anomaly(&current_vector, &profile.baseline_vector, cosine_threshold);
        }       
        println!("Warning: No baseline profile found for agent {}", agent_id);
        false
    }
}

