//! mesh_auditor_test.rs
//! Unit tests for the MeshAuditor.

#[cfg(test)]
mod tests {
    use kairo_rust_core::mesh_auditor::MeshAuditor;

    #[test]
    fn test_audit_flow_instantiation() {
        let auditor = MeshAuditor::new();
        let result = auditor.perform_audit("agent_001", &vec![1.0]);
        assert_eq!(result, false); // no profile, so no anomaly
    }
}
