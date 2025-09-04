//! mesh_auditor_test.rs
//! Unit tests for the MeshAuditor.

#[cfg(test)]
mod tests {
    use kairo_core::mesh_auditor::MeshAuditor;
    // Note: To make this test truly effective, we need to allow the auditor to access a mutable manager.
    // This will be addressed in the next refactoring phase.

    #[test]
    fn test_audit_flow_instantiation() {
        let auditor = MeshAuditor::new();
        // This initial test simply confirms the module compiles and can be instantiated.
        // A full integration test showing a mock anomaly detection will follow.
        let result = auditor.perform_audit("agent_001", &vec![1.0]);
        // Since no profile exists, it should return false (not an anomaly).
        assert_eq!(result, false);
    }
}
