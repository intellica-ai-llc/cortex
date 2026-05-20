use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Tracks business outcomes for Results‑as‑a‑Service billing.
pub struct OutcomeMetrics {
    counters: RwLock<HashMap<String, u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeReport {
    pub metric: String,
    pub count: u64,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
}

impl OutcomeMetrics {
    pub fn new() -> Self { Self { counters: RwLock::new(HashMap::new()) } }
    pub async fn increment(&self, metric: &str) {
        *self.counters.write().await.entry(metric.into()).or_default() += 1;
    }
    pub async fn report(&self, metric: &str) -> Option<OutcomeReport> {
        self.counters.read().await.get(metric).map(|&count| OutcomeReport {
            metric: metric.to_string(),
            count,
            period_start: chrono::Utc::now(),
            period_end: chrono::Utc::now(),
        })
    }
}
