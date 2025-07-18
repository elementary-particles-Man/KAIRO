// tests/conflict_resolver_test.rs

extern crate kairo;
use kairo::resolvers::conflict_resolver::{ConflictReport, LogicalConflictType, DefaultResolver, Resolution, ConflictResolver};

#[test]
fn test_contradiction_conflict_resolution() {
    let conflict = ConflictReport {
        timestamp: 1234567890,
        conflict_type: LogicalConflictType::Contradiction {
            node_ids: vec!["NodeA".into(), "NodeB".into()],
            conflicting_conclusions: vec!["True".into(), "False".into()],
        },
    };

    let resolver = DefaultResolver;
    let resolution = resolver.resolve(conflict);

    assert_eq!(resolution, Resolution::EscalateToHuman);
}
