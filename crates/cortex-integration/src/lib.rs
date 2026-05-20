//! Cortex IntegrationFabric — Universal enterprise connector surface.
//!
//! Auto-discovers, registers, and manages MCP/A2A connectors to every
//! enterprise system. Based on Peyrano's semantic gateway architecture
//! where every tool is dynamically discovered and authorised.

pub mod connector_registry;
pub mod openapi_generator;
pub mod schema_reverse_engineer;
pub mod legacy_adapter;
pub mod connectors;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top-level integration orchestrator.
pub struct IntegrationFabric {
    pub registry: Arc<connector_registry::ConnectorRegistry>,
    pub openapi_gen: openapi_generator::OpenAPIGenerator,
    pub schema_re: schema_reverse_engineer::SchemaReverseEngineer,
    pub legacy_adapter: legacy_adapter::LegacyAdapter,
}

impl IntegrationFabric {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(connector_registry::ConnectorRegistry::new()),
            openapi_gen: openapi_generator::OpenAPIGenerator::new(),
            schema_re: schema_reverse_engineer::SchemaReverseEngineer::new(),
            legacy_adapter: legacy_adapter::LegacyAdapter::new(),
        }
    }

    /// Auto-discover connectors on startup (v2 pattern).
    pub async fn auto_discover(&self) -> connector_registry::DiscoveryReport {
        self.registry.auto_discover().await
    }

    /// Generate MCP tools from an OpenAPI spec.
    pub fn generate_from_openapi(
        &self,
        spec: &str,
        base_url: &str,
    ) -> Result<Vec<connector_registry::ConnectorDefinition>, openapi_generator::GenerationError> {
        self.openapi_gen.generate(spec, base_url)
    }

    /// Reverse-engineer a JDBC/ODBC source into connector definitions.
    pub async fn reverse_engineer_jdbc(
        &self,
        connection_string: &str,
    ) -> Result<Vec<connector_registry::ConnectorDefinition>, schema_reverse_engineer::SchemaError> {
        self.schema_re.reverse_engineer_jdbc(connection_string).await
    }
}
