use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::RwLock;

/// Shadow MCP Detector — identifies unauthorised MCP servers.
///
/// Monitors all MCP traffic and identifies connections to servers
/// not registered in the Tool Registry. Unauthorised servers are
/// flagged, the connecting user is alerted, and the connection is
/// quarantined pending security review.
///
/// This implements the "Map the Shadows" pattern from the 2026
/// zero-trust guidance: "You can't secure what you can't see"[reference:9].
pub struct ShadowMCPDetector {
    /// Set of authorised MCP server endpoints.
    authorised_servers: RwLock<HashSet<String>>,
    /// Detected shadow servers.
    shadow_servers: RwLock<Vec<ShadowServer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowServer {
    pub endpoint: String,
    pub first_seen: chrono::DateTime<chrono::Utc>,
    pub connecting_user: String,
    pub risk_level: ShadowRiskLevel,
    pub quarantined: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShadowRiskLevel {
    Unknown,
    Suspicious,
    HighRisk,
    Blocked,
}

impl ShadowMCPDetector {
    pub fn new() -> Self {
        Self {
            authorised_servers: RwLock::new(HashSet::new()),
            shadow_servers: RwLock::new(Vec::new()),
        }
    }

    /// Register an authorised server endpoint.
    pub async fn register_authorised(&self, endpoint: &str) {
        self.authorised_servers.write().await.insert(endpoint.to_string());
    }

    /// Check if a server connection is authorised.
    pub async fn check_connection(
        &self,
        endpoint: &str,
        user: &str,
    ) -> Result<(), ShadowDetectionResult> {
        let authorised = self.authorised_servers.read().await;
        if authorised.contains(endpoint) {
            return Ok(());
        }

        // Shadow server detected
        let shadow = ShadowServer {
            endpoint: endpoint.to_string(),
            first_seen: chrono::Utc::now(),
            connecting_user: user.to_string(),
            risk_level: ShadowRiskLevel::Suspicious,
            quarantined: true,
        };

        let mut shadows = self.shadow_servers.write().await;
        shadows.push(shadow.clone());

        Err(ShadowDetectionResult {
            shadow,
            message: format!("Unauthorised MCP server detected: {}", endpoint),
        })
    }

    /// List all detected shadow servers.
    pub async fn list_shadows(&self) -> Vec<ShadowServer> {
        self.shadow_servers.read().await.clone()
    }
}

#[derive(Debug)]
pub struct ShadowDetectionResult {
    pub shadow: ShadowServer,
    pub message: String,
}
