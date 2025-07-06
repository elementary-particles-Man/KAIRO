use std::time::{Duration, Instant};
use rand_core::OsRng;
use x25519_dalek::{StaticSecret, PublicKey};

/// Represents an ephemeral Diffie-Hellman key pair that can be reused
/// for a short period of time to speed up exchanges.
pub struct SessionKey {
    pub private: StaticSecret,
    pub public: PublicKey,
    created: Instant,
}

impl SessionKey {
    fn new() -> Self {
        let private = StaticSecret::random_from_rng(&mut OsRng);
        let public = PublicKey::from(&private);
        Self { private, public, created: Instant::now() }
    }

    fn expired(&self, ttl: Duration) -> bool {
        self.created.elapsed() > ttl
    }
}

/// SessionManager manages ephemeral DH keys with a configurable TTL.
pub struct SessionManager {
    ttl: Duration,
    current: Option<SessionKey>,
}

impl SessionManager {
    /// Create a new manager with the given key Time-To-Live duration.
    pub fn new(ttl: Duration) -> Self {
        Self { ttl, current: None }
    }

    /// Obtain a DH key pair, reusing the existing one if it has not expired.
    pub fn keypair(&mut self) -> (&PublicKey, &StaticSecret) {
        if self.current.as_ref().map_or(true, |k| k.expired(self.ttl)) {
            self.current = Some(SessionKey::new());
        }
        let session = self.current.as_ref().unwrap();
        (&session.public, &session.private)
    }

    /// Compute a shared secret using the current private key and the peer's
    /// public key.
    pub fn shared_secret(&mut self, peer_public: &PublicKey) -> [u8; 32] {
        let (_, private) = self.keypair();
        private.diffie_hellman(peer_public).to_bytes()
    }
}
