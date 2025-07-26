use kairo_lib::resolvers::{ConflictReport, ConflictResolver, DefaultResolver, LogicalConflictType, Resolution};

#[test]
fn test_conflict_resolution() {
    let resolver = DefaultResolver;
    let report = ConflictReport {
        timestamp: 1234567890,
        conflict_type: LogicalConflictType::Contradiction {
            node_ids: vec!["node1".to_string(), "node2".to_string()],
            conflicting_conclusions: vec!["conclusionA".to_string(), "conclusionB".to_string()],
        },
    };
    let resolution = resolver.resolve(report);
    assert_eq!(resolution, Resolution::EscalateToHuman);
}
