//! Cortex Mesh™ — Autonomous Cross‑Enterprise Deployment (v7).
//!
//! Enables federated deployment of Cortex instances across multiple
//! enterprise sites, with A2A federation, federated learning, and
//! secure multi‑party computation for model updates.

pub mod auto_discovery;
pub mod federation_protocol;
pub mod federated_learning;
pub mod secure_aggregation;

use std::sync::Arc;

pub struct MeshEngine {
    pub discovery: Arc<auto_discovery::AutoDiscovery>,
    pub federation: Arc<federation_protocol::FederationProtocol>,
    pub fl: Arc<federated_learning::FederatedLearning>,
    pub aggregation: Arc<secure_aggregation::SecureAggregation>,
}

impl MeshEngine {
    pub fn new() -> Self {
        Self {
            discovery: Arc::new(auto_discovery::AutoDiscovery::new()),
            federation: Arc::new(federation_protocol::FederationProtocol::new()),
            fl: Arc::new(federated_learning::FederatedLearning::new()),
            aggregation: Arc::new(secure_aggregation::SecureAggregation::new()),
        }
    }
}
