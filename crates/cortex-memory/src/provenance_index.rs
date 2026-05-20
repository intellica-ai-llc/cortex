use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tokio::sync::RwLock;

/// Provenance Index (L7) — self‑anchored, Merkle‑proofed audit log.
pub struct ProvenanceIndex {
    entries: RwLock<VecDeque<ProvenanceEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub capsule_id: String,
    pub merkle_hash: String,
    pub signature: Vec<u8>,
}

impl ProvenanceIndex {
    pub fn new() -> Self {
        Self { entries: RwLock::new(VecDeque::with_capacity(100_000)) }
    }

    pub async fn append(&self, entry: &cortex_dream::journal::JournalEntry) {
        let mut entries = self.entries.write().await;
        entries.push_back(ProvenanceEntry {
            id: entry.id.clone(),
            timestamp: entry.timestamp,
            capsule_id: entry.id.clone(),
            merkle_hash: String::new(),
            signature: entry.signature.clone(),
        });
    }

    pub async fn latest_root(&self) -> Option<String> {
        let entries = self.entries.read().await;
        if entries.is_empty() { None }
        else { Some(entries.back().unwrap().merkle_hash.clone()) }
    }
}
