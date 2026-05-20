use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;

/// A registered enterprise connector definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorDefinition {
    pub id: String,
    pub name: String,
    pub system_type: SystemType,
    pub version: String,
    pub mcp_endpoint: Option<String>,
    pub openapi_spec_url: Option<String>,
    pub tools: Vec<ConnectorTool>,
    pub authentication: AuthConfig,
    pub rate_limits: RateLimits,
    pub status: ConnectorStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    SAP,
    Oracle,
    Salesforce,
    Workday,
    NetSuite,
    Dynamics365,
    ServiceNow,
    Snowflake,
    Jira,
    GitHub,
    Slack,
    Teams,
    SharePoint,
    Confluence,
    Database(String),
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub method: AuthMethod,
    pub client_id: Option<String>,
    pub tenant_id: Option<String>,
    pub token_url: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    OAuth2,
    Basic,
    ApiKey,
    Certificate,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    pub rpm: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectorStatus {
    Active,
    Degraded { reason: String },
    Disabled,
    Deprecated,
}

pub struct ConnectorRegistry {
    connectors: RwLock<HashMap<String, ConnectorDefinition>>,
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self { connectors: RwLock::new(HashMap::new()) }
    }

    pub async fn register(&self, def: ConnectorDefinition) {
        info!(id = %def.id, name = %def.name, "Registering connector");
        self.connectors.write().await.insert(def.id.clone(), def);
    }

    pub async fn get(&self, id: &str) -> Option<ConnectorDefinition> {
        self.connectors.read().await.get(id).cloned()
    }

    pub async fn list_all(&self) -> Vec<ConnectorDefinition> {
        self.connectors.read().await.values().cloned().collect()
    }

    pub async fn auto_discover(&self) -> DiscoveryReport {
        // In production: scan network for known endpoints, query .well-known/mcp.json
        DiscoveryReport { discovered: vec![], errors: vec![] }
    }
}

pub struct DiscoveryReport {
    pub discovered: Vec<String>,
    pub errors: Vec<String>,
}
