use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::time::Duration;

/// Periodic Base Snapshot — Merge on Read strategy.
///
/// The base snapshot is a periodically refreshed materialisation of
/// the CDC append log, optimised for agent read patterns. Agents
/// query the base snapshot for current state (sub‑ms reads via
/// primary key lookup), while the CDC append log preserves the
/// full immutable history.
///
/// Merge Strategy: Merge on Read (Pinterest recommendation).
/// "Copy on Write introduced significantly higher storage costs
/// because it rewrites entire data files during updates. Merge on
/// Read writes changes to separate files and applies them at read
/// time, reducing write amplification." (IOMETE, Apr 27, 2026).
///
/// For Cortex, this means: CDC events stream into the append log
/// continuously. Every `merge_interval` (configurable 1–60 min),
/// a merge operation combines the append log entries into the
/// base snapshot, producing a fresh point‑in‑time view.
pub struct BaseSnapshotManager {
    merge_interval: Duration,
    last_merge_at: tokio::sync::RwLock<Option<DateTime<Utc>>>,
    merge_strategy: MergeStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeStrategy {
    /// Merge on Read — writes changes to separate files, applies on read.
    MergeOnRead,
    /// Copy on Write — rewrites entire data files on update (faster reads, slower writes).
    CopyOnWrite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotState {
    pub source: String,
    pub table: String,
    pub snapshot_lsn: Option<String>,
    pub row_count: i64,
    pub last_merge_at: Option<DateTime<Utc>>,
    pub next_merge_at: Option<DateTime<Utc>>,
    pub merge_duration_ms: Option<u64>,
}

impl BaseSnapshotManager {
    pub fn new(merge_interval_minutes: u64) -> Self {
        Self {
            merge_interval: Duration::from_secs(merge_interval_minutes * 60),
            last_merge_at: tokio::sync::RwLock::new(None),
            merge_strategy: MergeStrategy::MergeOnRead,
        }
    }

    /// Check whether a merge is due.
    pub async fn is_merge_due(&self) -> bool {
        let last = *self.last_merge_at.read().await;
        match last {
            Some(t) => {
                let elapsed = Utc::now() - t;
                elapsed.to_std().unwrap_or(Duration::ZERO) >= self.merge_interval
            }
            None => true, // first merge is always due
        }
    }

    /// Record that a merge has been performed.
    pub async fn record_merge(&self) {
        *self.last_merge_at.write().await = Some(Utc::now());
    }

    /// Get the current merge strategy.
    pub fn strategy(&self) -> &MergeStrategy { &self.merge_strategy }

    /// Estimate write amplification for a given strategy.
    /// MoR: 1.05× (small write amplification). CoW: 3–5× (full file rewrite).
    pub fn write_amplification(&self) -> f64 {
        match self.merge_strategy {
            MergeStrategy::MergeOnRead => 1.05,
            MergeStrategy::CopyOnWrite => 4.0,
        }
    }
}
