use crate::connector_registry::*;
pub struct OracleConnector;
impl OracleConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "oracle-fusion".into(), name: "Oracle Fusion Cloud".into(), system_type: SystemType::Oracle, version: "24D".into(),
            mcp_endpoint: Some("https://oracle.internal/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "oracle_get_employee".into(), description: "Get employee record".into(), input_schema: serde_json::json!({"person_id": "string"}), output_schema: None },
                ConnectorTool { name: "oracle_create_po".into(), description: "Create procurement order".into(), input_schema: serde_json::json!({"supplier": "string", "amount": "number"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://oracle.internal/oauth/token".into()), scopes: vec!["urn:opc:resource:consumer::all".into()] },
            rate_limits: RateLimits { rpm: 100, burst_size: 8 }, status: ConnectorStatus::Active,
        }
    }
}
