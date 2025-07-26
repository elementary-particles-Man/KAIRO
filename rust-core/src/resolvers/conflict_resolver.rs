#[derive(Debug, PartialEq)]
pub enum LogicalConflictType {
    Contradiction {
        node_ids: Vec<String>,
        conflicting_conclusions: Vec<String>,
    },
}

#[derive(Debug)]
pub struct ConflictReport {
    pub timestamp: u64,
    pub conflict_type: LogicalConflictType,
}

#[derive(Debug, PartialEq)]
pub enum Resolution {
    EscalateToHuman,
    AutoResolved(String),
}

pub trait ConflictResolver {
    fn resolve(&self, report: ConflictReport) -> Resolution;
}

pub struct DefaultResolver;

impl ConflictResolver for DefaultResolver {
    fn resolve(&self, _report: ConflictReport) -> Resolution {
        Resolution::EscalateToHuman
    }
}
