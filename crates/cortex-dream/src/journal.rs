use ed25519_dalek::{SigningKey, Signer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Append‑only dream journal with ed25519 signing.
pub struct JournalWriter {
    signing_key: SigningKey,
    entries: Arc<Mutex<Vec<JournalEntry>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub signature: Vec<u8>,
    pub previous_entry_hash: Option<String>,
}

impl JournalWriter {
    pub fn new(signing_key: [u8; 32]) -> Self {
        let key = SigningKey::from_bytes(&signing_key);
        Self {
            signing_key: key,
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Sign a new journal entry.
    pub async fn sign_entry(&self, description: &str) -> JournalEntry {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();

        // Link to previous entry for tamper resistance.
        let prev_hash = {
            let entries = self.entries.lock().await;
            entries.last().map(|e| e.signature.iter().map(|b| format!("{:02x}", b)).collect::<String>())
        };

        let payload = format!("{}:{}:{}", id, timestamp.to_rfc3339(), description);
        let signature = self.signing_key.sign(payload.as_bytes()).to_vec();

        let entry = JournalEntry {
            id,
            timestamp,
            description: description.to_string(),
            signature: signature.clone(),
            previous_entry_hash: prev_hash,
        };

        self.entries.lock().await.push(entry.clone());
        entry
    }
}
