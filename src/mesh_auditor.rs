//! mesh_auditor.rs
//! Periodically audits agent behavior using the integrated diagnosis system.

use crate::baseline_profile_manager::BaselineProfileManager;
use crate::mesh_trust_calculator::TrustScoreCalculator;

#[derive(Debug)]
pub struct MeshAuditor {
    profile_manager: BaselineProfileManager,
    trust_calculator: TrustScoreCalculator,
}

impl MeshAuditor {
    pub fn new() -> Self {
        Self {
            profile_manager: BaselineProfileManager::new(),
            trust_calculator: TrustScoreCalculator::new(),
        }
    }

    // This is the main audit loop function.
    pub fn perform_audit(&self, agent_id: &str, current_vector: &[f64]) -> bool {
        // For now, we use a fixed threshold. This could be dynamic later.
        let cosine_threshold = 0.95;

        // The core logic: verify the agent's behavior using the integrated system.
        let is_anomaly = self.trust_calculator.verify_agent_behavior(
            &self.profile_manager,
            agent_id,
            current_vector,
            cosine_threshold
        );

        if is_anomaly {
            println!("AUDIT FAILED: Anomaly detected for agent {}", agent_id);
        } else {
            println!("AUDIT PASSED: No anomaly detected for agent {}", agent_id);
        }

        is_anomaly
    }
}
