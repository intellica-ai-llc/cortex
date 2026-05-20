use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Federated Store (L5) — CRDT‑backed cross‑instance sharing.
pub struct FederatedStore {
    // CRDT state would be managed via ElectricSQL or similar.
    pending_syncs: RwLock<HashMap<String, FederatedRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedRecord {
    pub key: String,
    pub value: serde_json::Value,
    pub vector_clock: u64,
}

impl FederatedStore {
    pub fn new() -> Self {
        Self { pending_syncs: RwLock::new(HashMap::new()) }
    }

    pub async fn put(&self, key: &str, value: serde_json::Value) {
        self.pending_syncs.write().await.insert(key.to_string(), FederatedRecord {
            key: key.to_string(),
            value,
            vector_clock: 1,
        });
    }

    pub async fn get(&self, key: &str) -> Option<serde_json::Value> {
        self.pending_syncs.read().await.get(key).map(|r| r.value.clone())
    }
}
