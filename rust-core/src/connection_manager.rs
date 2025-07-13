use std::collections::HashMap;
use std::time::Duration;

use kairo_core::session_reuse::SessionManager;

/// Manage ephemeral sessions for active connections.
pub struct ConnectionManager {
    ttl: Duration,
    sessions: HashMap<u32, SessionManager>,
}

impl ConnectionManager {
    /// Create a new manager with the session TTL.
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            sessions: HashMap::new(),
        }
    }

    /// Establish a connection with the given identifier.
    pub fn connect(&mut self, id: u32) {
        self.sessions
            .entry(id)
            .or_insert_with(|| SessionManager::new());
    }

    /// Remove the connection, returning true if it existed.
    pub fn disconnect(&mut self, id: u32) -> bool {
        self.sessions.remove(&id).is_some()
    }

    /// Check whether a connection with the identifier exists.
    pub fn is_connected(&self, id: u32) -> bool {
        self.sessions.contains_key(&id)
    }

    /// Number of active connections.
    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_and_is_connected() {
        let mut mgr = ConnectionManager::new(Duration::from_secs(5));
        mgr.connect(42);
        assert!(mgr.is_connected(42));
        assert_eq!(mgr.count(), 1);

        // connecting again should not create duplicate sessions
        mgr.connect(42);
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_disconnect() {
        let mut mgr = ConnectionManager::new(Duration::from_secs(5));
        mgr.connect(1);
        mgr.connect(2);
        assert_eq!(mgr.count(), 2);

        assert!(mgr.disconnect(1));
        assert!(!mgr.is_connected(1));
        assert_eq!(mgr.count(), 1);

        // disconnecting a non-existent id should return false
        assert!(!mgr.disconnect(3));
    }
}
