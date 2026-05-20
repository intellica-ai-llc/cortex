use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tokio::sync::RwLock;

/// Episodic Store (L1) — recent event log with temporal chain.
pub struct EpisodicStore {
    traces: RwLock<VecDeque<TraceEntry>>,
    max_capacity: usize,
    total_stored: tokio::sync::Mutex<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub source_application: String,
    pub field_path: String,
    pub behavioral_token: String,
    pub importance: f64,       // 0.0 (trivial) to 1.0 (critical)
    pub access_count: u32,     // reinforcement count
}

impl EpisodicStore {
    pub fn new() -> Self {
        Self {
            traces: RwLock::new(VecDeque::with_capacity(10_000)),
            max_capacity: 10_000,
            total_stored: tokio::sync::Mutex::new(0),
        }
    }

    /// Append a new episodic trace.
    pub async fn append(&self, trace: TraceEntry) {
        let mut traces = self.traces.write().await;
        if traces.len() >= self.max_capacity {
            traces.pop_front(); // evict oldest
        }
        traces.push_back(trace);
        *self.total_stored.lock().await += 1;
    }

    /// Return the most recent N traces.
    pub async fn recent_traces(&self, n: usize) -> Vec<TraceEntry> {
        let traces = self.traces.read().await;
        traces.iter().rev().take(n).cloned().collect()
    }

    /// Prune traces whose importance has decayed below a threshold.
    /// Returns (pruned count, retained count).
    pub async fn prune_aged(&self, threshold: f64) -> (usize, usize) {
        let mut traces = self.traces.write().await;
        let before = traces.len();
        traces.retain(|t| t.importance > threshold);
        (before - traces.len(), traces.len())
    }

    /// Reinforce a trace (increase importance and access count).
    pub async fn reinforce(&self, trace_id: &str) {
        let mut traces = self.traces.write().await;
        if let Some(t) = traces.iter_mut().find(|t| t.id == trace_id) {
            t.importance = (t.importance + 0.1).min(1.0);
            t.access_count += 1;
        }
    }

    /// Estimate total memory usage in bytes.
    pub fn memory_estimate(&self) -> u64 {
        self.traces.try_read().map(|t| t.len() as u64 * 500).unwrap_or(0)
    }
}
