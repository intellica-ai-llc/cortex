use async_trait::async_trait;
use super::super::connector::{Connector, ConnectorError, ConnectorTool};

pub struct GitHubConnector { client: Option<reqwest::Client> }

impl GitHubConnector {
    pub fn new() -> Self {
        let token = std::env::var("GITHUB_TOKEN").ok();
        let client = match token {
            Some(t) => {
                let mut headers = reqwest::header::HeaderMap::new();
                if let Ok(val) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", t)) {
                    headers.insert(reqwest::header::AUTHORIZATION, val);
                }
                Some(reqwest::Client::builder().default_headers(headers).build().unwrap_or_else(|_| reqwest::Client::new()))
            }
            None => None,
        };
        Self { client }
    }
    pub fn is_configured(&self) -> bool { self.client.is_some() }
}

#[async_trait]
impl Connector for GitHubConnector {
    fn name(&self) -> &str { "github" }
    fn tools(&self) -> Vec<ConnectorTool> {
        vec![ConnectorTool { name: "github_list_prs".into(), description: "List pull requests for a repository".into(), input_schema: serde_json::json!({"type":"object","properties":{"owner":{"type":"string"},"repo":{"type":"string"}},"required":["owner","repo"]}), output_schema: Some(serde_json::json!({"type":"array","items":{"type":"object"}})) }]
    }
    async fn execute(&self, tool_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ConnectorError> {
        let _client = self.client.as_ref().ok_or_else(|| ConnectorError::AuthFailed("GITHUB_TOKEN not set".into()))?;
        match tool_name {
            "github_list_prs" => {
                let _owner = params.get("owner").and_then(|v| v.as_str()).unwrap_or("intellica-ai-llc");
                let _repo = params.get("repo").and_then(|v| v.as_str()).unwrap_or("cortex");
                Ok(serde_json::json!({"message":"GitHub connector (MVP)","status":"ok"}))
            }
            _ => Err(ConnectorError::ToolNotFound(tool_name.to_string())),
        }
    }
}
