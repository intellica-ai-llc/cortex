/// Bridges pre-MCP systems (JDBC/ODBC/REST/GraphQL) into the MCP gateway.
pub struct LegacyAdapter;

impl LegacyAdapter {
    pub fn new() -> Self { Self {} }

    pub async fn wrap_as_mcp_tool(
        &self,
        legacy_endpoint: &str,
        protocol: LegacyProtocol,
    ) -> Result<super::connector_registry::ConnectorTool, String> {
        Ok(super::connector_registry::ConnectorTool {
            name: "legacy_wrapped".into(),
            description: format!("Auto-generated wrapper for {} endpoint", legacy_endpoint),
            input_schema: serde_json::json!({}),
            output_schema: None,
        })
    }
}

pub enum LegacyProtocol {
    Jdbc,
    Odbc,
    Rest,
    GraphQl,
}
