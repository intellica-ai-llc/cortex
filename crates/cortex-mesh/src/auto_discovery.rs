use serde::{Deserialize, Serialize};

pub struct AutoDiscovery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredNode {
    pub node_id: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
}

impl AutoDiscovery {
    pub fn new() -> Self { Self }
    pub async fn scan_network(&self) -> Vec<DiscoveredNode> { vec![] }
}
