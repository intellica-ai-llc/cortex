use serde::{Deserialize, Serialize};

pub struct FederationProtocol;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationConfig {
    pub node_did: String,
    pub a2a_endpoint: String,
}

impl FederationProtocol {
    pub fn new() -> Self { Self }
    pub fn bootstrap(&self, node_did: &str, a2a_endpoint: &str) -> FederationConfig {
        FederationConfig { node_did: node_did.into(), a2a_endpoint: a2a_endpoint.into() }
    }
}
