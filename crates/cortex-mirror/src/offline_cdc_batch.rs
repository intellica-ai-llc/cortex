use serde::{Deserialize, Serialize};

/// Offline CDC Batch Mode — air‑gapped deployment support.
///
/// When the target Cortex instance is physically isolated, a
/// sidecar inside the source network captures CDC events, encrypts
/// them, and packages them as signed artifacts. The batch file is
/// then transferred via one‑way diode or physical media and
/// ingested into TraceDB.
pub struct OfflineCdcBatchEngine {
    batch_dir: String,
    encryption_key_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdcBatchManifest {
    pub source: String,
    pub batch_id: String,
    pub start_lsn: String,
    pub end_lsn: String,
    pub event_count: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub signature: Option<Vec<u8>>,
}

impl OfflineCdcBatchEngine {
    pub fn new(batch_dir: &str, encryption_key_id: Option<&str>) -> Self {
        Self {
            batch_dir: batch_dir.to_string(),
            encryption_key_id: encryption_key_id.map(|s| s.to_string()),
        }
    }

    pub async fn create_batch(
        &self,
        source: &str,
        events: &[super::cdc_append_log::CdcLogEntry],
    ) -> Result<CdcBatchManifest, String> {
        let batch_id = uuid::Uuid::new_v4().to_string();
        // Production: serialise events, encrypt if key_id present,
        // write to batch_dir, and sign with Cortex instance key.
        Ok(CdcBatchManifest {
            source: source.to_string(),
            batch_id,
            start_lsn: events.first().and_then(|e| e.lsn.clone()).unwrap_or_default(),
            end_lsn: events.last().and_then(|e| e.lsn.clone()).unwrap_or_default(),
            event_count: events.len() as u64,
            created_at: chrono::Utc::now(),
            signature: None,
        })
    }
}
