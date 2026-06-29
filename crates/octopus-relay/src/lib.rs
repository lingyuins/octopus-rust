pub mod balancer;
pub mod handler;
pub mod condition;
pub mod circuit_breaker;
pub mod semantic_cache;

/// Relay handler - processes LLM API requests through the channel system
pub struct RelayHandler;

impl RelayHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RelayHandler {
    fn default() -> Self {
        Self::new()
    }
}