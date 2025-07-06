use std::collections::HashSet;

/// Simple firewall filter based on integer rules.
#[derive(Default)]
pub struct Firewall {
    deny_rules: HashSet<u32>,
}

impl Firewall {
    /// Create a new firewall instance.
    pub fn new() -> Self {
        Self { deny_rules: HashSet::new() }
    }

    /// Add an identifier to the deny list.
    pub fn deny(&mut self, id: u32) {
        self.deny_rules.insert(id);
    }

    /// Remove an identifier from the deny list.
    pub fn allow(&mut self, id: u32) {
        self.deny_rules.remove(&id);
    }

    /// Check whether the given identifier is allowed.
    pub fn is_allowed(&self, id: u32) -> bool {
        !self.deny_rules.contains(&id)
    }
}
