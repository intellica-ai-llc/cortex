use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Dual‑Write Propagation Engine — mirrors writes back to source.
///
/// Based on Gusto’s “Double Write Methodology”: write data to both
/// the old and new tables. After reads move exclusively to the new
/// table, stop writing to the old. The legacy system remains fully
/// synchronised and sees normal write volumes, masking the migration.
///
/// Rownd’s staged migration adds: “Each migration followed a staged
/// process: read/write primary → read primary, write all → read all,
/// write primary → complete cutover.”
pub struct MirrorDualWritePropagator {
    /// Active dual‑write sessions per source.
    active_writes: RwLock<HashMap<String, Vec<DualWriteRecord>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualWriteRecord {
    pub id: String,
    pub source: String,
    pub table: String,
    pub primary_key: String,
    pub new_values: serde_json::Value,
    pub legacy_write_status: WriteStatus,
    pub tracedb_write_status: WriteStatus,
    pub initiated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WriteStatus {
    Pending,
    Success,
    Failed { reason: String },
}

impl MirrorDualWritePropagator {
    pub fn new() -> Self {
        Self { active_writes: RwLock::new(HashMap::new()) }
    }

    /// Propagate a write to both TraceDB and the source system.
    pub async fn propagate(
        &self,
        source: &str,
        table: &str,
        primary_key: &str,
        new_values: serde_json::Value,
    ) -> DualWriteRecord {
        // In production: write to TraceDB first, then issue MCP/JDBC write
        // back to the legacy system. If the legacy write fails, mark for retry.
        let record = DualWriteRecord {
            id: uuid::Uuid::new_v4().to_string(),
            source: source.to_string(),
            table: table.to_string(),
            primary_key: primary_key.to_string(),
            new_values,
            legacy_write_status: WriteStatus::Pending,
            tracedb_write_status: WriteStatus::Success,
            initiated_at: chrono::Utc::now(),
        };
        self.active_writes.write().await
            .entry(source.to_string())
            .or_default()
            .push(record.clone());
        record
    }

    pub async fn pending_for_source(&self, source: &str) -> Vec<DualWriteRecord> {
        self.active_writes.read().await
            .get(source)
            .cloned()
            .unwrap_or_default()
    }
}
