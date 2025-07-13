//! Session.rs
//! Ephemeral Key + Scope

pub struct EphemeralSession {
    pub session_id: String,
    pub ephemeral_key: String,
    pub scope: Scope,
}

pub enum Scope {
    Personal,
    Family,
    Group,
    Community,
    World,
}

impl EphemeralSession {
    pub fn resume_session(session_id: &str, key: &str, scope: Scope) -> Self {
        Self {
            session_id: session_id.to_string(),
            ephemeral_key: key.to_string(),
            scope,
        }
    }
}
