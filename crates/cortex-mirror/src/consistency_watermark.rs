use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Cross‑Source Consistency Watermark — Flink CDC pattern.
///
/// Flink CDC 3.6.0 uses low‑watermark and high‑watermark mechanisms
/// to ensure transactional consistency across multiple source
/// connectors (FLIP‑326, Apr 2026). When all active CDC pipelines
/// complete a full sync cycle, the Mirror Engine generates a
/// consistency watermark — a timestamp representing the latest
/// point at which all mirrored data across all sources is mutually
/// consistent.
///
/// Agents executing cross‑source queries (e.g., joining Workday
/// employee data with Snowflake analytics) check this watermark.
/// If the agent needs data fresher than the watermark, it routes
/// through live MCP connectors.
pub struct CrossSourceWatermark {
    /// source_name → latest LSN / timestamp.
    watermarks: tokio::sync::RwLock<std::collections::HashMap<String, SourceWatermark>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceWatermark {
    pub source: String,
    pub latest_lsn: Option<String>,
    pub latest_timestamp: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

/// The global consistency watermark — the minimum across all sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalWatermark {
    /// The timestamp representing the point where all sources are consistent.
    pub consistent_at: Option<DateTime<Utc>>,
    /// Per‑source details.
    pub sources: Vec<SourceWatermark>,
    /// Whether any source is lagging.
    pub all_sources_consistent: bool,
}

impl CrossSourceWatermark {
    pub fn new() -> Self {
        Self { watermarks: tokio::sync::RwLock::new(std::collections::HashMap::new()) }
    }

    /// Update the watermark for a single source.
    pub async fn update(&self, source: &str, lsn: Option<&str>, ts: DateTime<Utc>) {
        self.watermarks.write().await.insert(source.to_string(), SourceWatermark {
            source: source.to_string(),
            latest_lsn: lsn.map(|s| s.to_string()),
            latest_timestamp: Some(ts),
            updated_at: Utc::now(),
        });
    }

    /// Compute the global watermark: the minimum timestamp across all sources.
    /// This is the point at which all sources are guaranted mutually consistent.
    pub async fn global(&self) -> GlobalWatermark {
        let sources: Vec<SourceWatermark> = self.watermarks.read().await.values().cloned().collect();
        let consistent_at = sources.iter()
            .filter_map(|s| s.latest_timestamp)
            .min();

        GlobalWatermark {
            consistent_at,
            sources: sources.clone(),
            all_sources_consistent: sources.iter().all(|s| s.latest_timestamp.is_some()),
        }
    }

    /// Check whether an agent query with a given recency requirement can be
    /// served from TraceDB, or whether it must route to the live source.
    pub async fn can_serve_from_tracedb(&self, max_age: chrono::Duration) -> bool {
        let global = self.global().await;
        match global.consistent_at {
            Some(ts) => {
                let age = Utc::now() - ts;
                age.to_std().unwrap_or(chrono::Duration::max_value()) <= max_age
            }
            None => true, // no sources registered yet — serve from TraceDB
        }
    }
}
