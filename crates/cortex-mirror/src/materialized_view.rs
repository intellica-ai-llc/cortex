use serde::{Deserialize, Serialize};

/// RisingWave‑style continuously refreshed materialized views.
///
/// RisingWave (Apr 2, 2026): "You create materialized views that
/// are incrementally maintained as new data arrives. RisingWave
/// supports complex multi‑way joins, window functions, temporal
/// filters, and sub‑queries in a streaming context."
///
/// The critical design decision for Cortex Mirror: agents should
/// not query CDC streams directly. They should query materialized
/// views that are pre‑computed from those streams. The rationale
/// (from the Streamkap agent‑ready store pattern): "For complex
/// context — joins across 5 tables, aggregations over 30‑day
/// windows — each API call would be expensive. Streaming
/// materialized views pre‑compute this context once and serve it
/// instantly."
///
/// Every absorbed field creates a corresponding materialized view
/// in TraceDB. The view is continuously refreshed from the CDC
/// append log. When the Converge Controller queries "what is the
/// current status of work order XYZ?", it reads from the
/// materialized view — not from the CDC log, not from the source
/// system, not from an API call.
pub struct MaterializedViewManager {
    /// Registered views indexed by (source, table).
    views: tokio::sync::RwLock<std::collections::HashMap<String, MaterializedViewDef>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterializedViewDef {
    pub view_name: String,
    pub source_table: String,
    pub refresh_mode: RefreshMode,
    pub last_refresh_at: Option<chrono::DateTime<chrono::Utc>>,
    pub row_count: i64,
    pub freshness_ms: u64,            // milliseconds since last refresh
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefreshMode {
    /// Continuously updated as CDC events arrive (sub‑100ms).
    Continuous,
    /// Refreshed on a fixed schedule (e.g., every 1 second).
    Scheduled { interval_ms: u64 },
    /// Refreshed on demand (lazy).
    OnDemand,
}

impl MaterializedViewManager {
    pub fn new() -> Self {
        Self { views: tokio::sync::RwLock::new(std::collections::HashMap::new()) }
    }

    /// Register a new materialized view backed by an absorption table.
    pub async fn register(&self, view_name: &str, source_table: &str, mode: RefreshMode) {
        self.views.write().await.insert(view_name.to_string(), MaterializedViewDef {
            view_name: view_name.to_string(),
            source_table: source_table.to_string(),
            refresh_mode: mode,
            last_refresh_at: None,
            row_count: 0,
            freshness_ms: 0,
        });
    }

    /// Get the freshness of a view in milliseconds.
    pub async fn freshness_ms(&self, view_name: &str) -> Option<u64> {
        self.views.read().await.get(view_name).map(|v| v.freshness_ms)
    }

    /// Check if a view is fresh enough for agent queries (< 100ms for real‑time).
    pub async fn is_fresh(&self, view_name: &str, max_age_ms: u64) -> bool {
        self.views.read().await.get(view_name)
            .map(|v| v.freshness_ms <= max_age_ms)
            .unwrap_or(false)
    }
}
