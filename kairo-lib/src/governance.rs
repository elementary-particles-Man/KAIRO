//! kairo-lib/src/governance.rs
//! Defines the structures and protocols for identity sovereignty.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReissueRequestPayload {
    pub old_agent_id: String,
    pub new_agent_id: String,
    pub reason: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignaturePackage {
    pub signatory_id: String, // ID of the entity signing (Peer AI, Seed Node, Human Auditor)
    pub signature: String,    // Digital signature of the payload
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OverridePackage {
    pub payload: ReissueRequestPayload,
    pub signatures: Vec<SignaturePackage>,
}
