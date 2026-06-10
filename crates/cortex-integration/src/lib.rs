pub mod connector_registry;
pub mod openapi_generator;
pub mod schema_reverse_engineer;
pub mod legacy_adapter;
pub mod connectors;
pub mod connector;

use std::sync::Arc;

pub struct IntegrationFabric {
    pub registry: Arc<connector_registry::ConnectorRegistry>,
    pub openapi_gen: openapi_generator::OpenAPIGenerator,
    pub schema_re: schema_reverse_engineer::SchemaReverseEngineer,
    pub legacy_adapter: legacy_adapter::LegacyAdapter,
    pub connector_registry: Arc<connector::ConnectorRegistry>,
}

impl IntegrationFabric {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(connector_registry::ConnectorRegistry::new()),
            openapi_gen: openapi_generator::OpenAPIGenerator::new(),
            schema_re: schema_reverse_engineer::SchemaReverseEngineer::new(),
            legacy_adapter: legacy_adapter::LegacyAdapter::new(),
            connector_registry: Arc::new(connector::ConnectorRegistry::new()),
        }
    }
}
