use async_trait::async_trait;
use super::super::connector::{Connector, ConnectorError, ConnectorTool};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct PostgresConnector { pool: Option<PgPool> }

impl PostgresConnector {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").ok();
        let pool = match database_url {
            Some(url) => match PgPoolOptions::new().max_connections(5).connect(&url).await {
                Ok(p) => Some(p),
                Err(e) => { tracing::warn!("PostgreSQL not available: {}", e); None }
            },
            None => None,
        };
        Self { pool }
    }
    pub fn is_connected(&self) -> bool { self.pool.is_some() }
}

#[async_trait]
impl Connector for PostgresConnector {
    fn name(&self) -> &str { "postgres" }
    fn tools(&self) -> Vec<ConnectorTool> {
        vec![
            ConnectorTool { name: "postgres_query".into(), description: "Execute a SQL query against PostgreSQL".into(), input_schema: serde_json::json!({"type":"object","properties":{"query":{"type":"string"}},"required":["query"]}), output_schema: Some(serde_json::json!({"type":"array","items":{"type":"object"}})) },
            ConnectorTool { name: "postgres_list_tables".into(), description: "List all tables in PostgreSQL".into(), input_schema: serde_json::json!({"type":"object","properties":{}}), output_schema: Some(serde_json::json!({"type":"array","items":{"type":"object"}})) },
        ]
    }
    async fn execute(&self, tool_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ConnectorError> {
        let pool = self.pool.as_ref().ok_or_else(|| ConnectorError::ExecutionFailed("No database connection".into()))?;
        match tool_name {
            "postgres_list_tables" => {
                let rows: Vec<(String,)> = sqlx::query_as("SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname = 'public'").fetch_all(pool).await.map_err(|e| ConnectorError::ExecutionFailed(e.to_string()))?;
                Ok(serde_json::Value::Array(rows.into_iter().map(|(n,)| serde_json::json!({"table":n})).collect()))
            }
            "postgres_query" => {
                let query = params.get("query").and_then(|v| v.as_str()).ok_or_else(|| ConnectorError::ExecutionFailed("Missing 'query'".into()))?;
                let trimmed = query.trim().to_uppercase();
                if !trimmed.starts_with("SELECT") && !trimmed.starts_with("WITH") { return Err(ConnectorError::ExecutionFailed("Only SELECT queries permitted".into())); }
                let rows = sqlx::query_as::<_, (serde_json::Value,)>(query).fetch_all(pool).await.map_err(|e| ConnectorError::ExecutionFailed(e.to_string()))?;
                Ok(serde_json::Value::Array(rows.into_iter().map(|(v,)| v).collect()))
            }
            _ => Err(ConnectorError::ToolNotFound(tool_name.to_string())),
        }
    }
}
