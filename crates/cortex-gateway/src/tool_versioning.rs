use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Semantic versioning & drift detection for MCP tools.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ToolVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

pub struct ToolVersionRegistry {
    versions: RwLock<HashMap<String, ToolVersion>>,
}

impl ToolVersionRegistry {
    pub fn new() -> Self {
        Self { versions: RwLock::new(HashMap::new()) }
    }

    pub async fn register(&self, tool_id: &str, version: ToolVersion) {
        self.versions.write().await.insert(tool_id.into(), version);
    }

    pub async fn check_drift(&self, tool_id: &str, observed_version: &ToolVersion) -> bool {
        if let Some(registered) = self.versions.read().await.get(tool_id) {
            registered != observed_version
        } else {
            false
        }
    }
}
