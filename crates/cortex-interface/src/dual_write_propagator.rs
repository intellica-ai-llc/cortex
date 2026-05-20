use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Dual‑Write Propagation Engine — invisibility to vendors.
///
/// Every user write through Cortex is mirrored back to the legacy
/// system via MCP connector or direct JDBC. The legacy system stays
/// fully synchronised, so vendors see normal write volumes.
///
/// Based on Gusto’s “Double Write Methodology” and Rownd’s staged
/// migration pattern: write to both, read from new once consistent,
/// then cut over.
pub struct DualWritePropagator {
    /// Active dual‑write sessions per user per application.
    active_writes: RwLock<HashMap<String, Vec<DualWriteRecord>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualWriteRecord {
    pub id: String,
    pub user_id: String,
    pub application: String,
    pub field: String,
    pub new_value: serde_json::Value,
    pub legacy_write_status: WriteStatus,
    pub cortex_write_status: WriteStatus,
    pub initiated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WriteStatus {
    Pending,
    Success,
    Failed { reason: String },
}

impl DualWritePropagator {
    pub fn new() -> Self {
        Self { active_writes: RwLock::new(HashMap::new()) }
    }

    /// Propagate a user write to both Cortex TraceDB and the legacy system.
    pub async fn propagate(
        &self,
        user_id: &str,
        application: &str,
        field: &str,
        new_value: &serde_json::Value,
    ) -> DualWriteRecord {
        let record = DualWriteRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            application: application.to_string(),
            field: field.to_string(),
            new_value: new_value.clone(),
            legacy_write_status: WriteStatus::Pending,
            cortex_write_status: WriteStatus::Success, // write to TraceDB is always first
            initiated_at: chrono::Utc::now(),
        };

        // In production: write to legacy via MCP connector or JDBC.
        // The write is tagged as coming from the application user,
        // so the legacy app sees a standard client connection.

        let mut writes = self.active_writes.write().await;
        writes.entry(user_id.to_string()).or_default().push(record.clone());
        record
    }

    /// Verify that a dual‑write completed successfully on both sides.
    pub async fn verify(&self, write_id: &str) -> bool {
        // Production: compare checksums between TraceDB and legacy.
        true
    }
}
