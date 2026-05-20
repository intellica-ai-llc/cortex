use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Column‑Level CDC filter — only replicate columns users actually access.
///
/// Based on Popsink’s 2026 CDC guide: “Map the Data: Select which
/// tables and columns you want to replicate.” and SQL Server CDC’s
/// native column‑selection feature. This filter sits between the CDC
/// backend and TraceDB, reducing data volume by replicating only the
/// columns the Observational Agent has discovered as relevant.
pub struct ColumnLevelCdcFilter {
    /// Table → set of columns to replicate.
    column_allowlist: tokio::sync::RwLock<std::collections::HashMap<String, HashSet<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdcEvent {
    pub source: String,
    pub table: String,
    pub operation: CdcOperation,
    pub columns: serde_json::Value,     // {column: new_value}
    pub transaction_id: String,
    pub lsn: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CdcOperation { Insert, Update, Delete }

impl ColumnLevelCdcFilter {
    pub fn new() -> Self {
        Self { column_allowlist: tokio::sync::RwLock::new(std::collections::HashMap::new()) }
    }

    /// Register columns to replicate for a table.
    pub async fn allow_columns(&self, table: &str, columns: Vec<String>) {
        let mut map = self.column_allowlist.write().await;
        let entry = map.entry(table.to_string()).or_default();
        for col in columns { entry.insert(col); }
    }

    /// Filter a raw CDC event to only allowed columns.
    pub async fn filter(&self, event: &CdcEvent) -> CdcEvent {
        let map = self.column_allowlist.read().await;
        if let Some(allowed) = map.get(&event.table) {
            if let serde_json::Value::Object(obj) = &event.columns {
                let filtered: serde_json::Map<String, serde_json::Value> = obj
                    .iter()
                    .filter(|(k, _)| allowed.contains(*k))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                return CdcEvent {
                    columns: serde_json::Value::Object(filtered),
                    ..event.clone()
                };
            }
        }
        event.clone()
    }
}
