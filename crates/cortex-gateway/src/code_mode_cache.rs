use std::collections::HashMap;
use tokio::sync::RwLock;

/// Cloudflare Code Mode compatibility:
/// Reduces token costs by sending only function names & params after initial discovery.
pub struct CodeModeCache {
    inner: RwLock<HashMap<String, CachedToolDescription>>,
}

struct CachedToolDescription {
    description: String,
    schema_hash: String,
}

impl CodeModeCache {
    pub fn new() -> Self {
        Self { inner: RwLock::new(HashMap::new()) }
    }

    pub async fn cache_tool(&self, tool_id: &str, description: &str, schema_hash: &str) {
        let mut lock = self.inner.write().await;
        lock.insert(tool_id.to_string(), CachedToolDescription {
            description: description.to_string(),
            schema_hash: schema_hash.to_string(),
        });
    }

    pub async fn get_tool(&self, tool_id: &str) -> Option<String> {
        let lock = self.inner.read().await;
        lock.get(tool_id).map(|c| c.description.clone())
    }
}
