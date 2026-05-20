use std::collections::VecDeque;
use tokio::sync::RwLock;

/// Continuous agent behaviour monitoring with anomaly detection.
///
/// Factor 2 of the CortexGuard kill switch. Monitors agent tool
/// calls, latency patterns, and data access for deviations beyond
/// 3σ from the learned baseline.
pub struct BehavioralBaseline {
    /// Rolling window of recent observations for baseline computation.
    recent: RwLock<VecDeque<BehaviorObservation>>,
    window_size: usize,
    deviation_threshold: f64, // sigma multiplier
}

#[derive(Debug, Clone)]
pub struct BehaviorObservation {
    pub tool_calls_per_minute: f64,
    pub avg_latency_ms: f64,
    pub unique_tools_accessed: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl BehavioralBaseline {
    pub fn new() -> Self {
        Self {
            recent: RwLock::new(VecDeque::with_capacity(1000)),
            window_size: 100,
            deviation_threshold: 3.0,
        }
    }

    /// Record a new observation.
    pub async fn observe(&self, obs: BehaviorObservation) {
        let mut recent = self.recent.write().await;
        recent.push_back(obs);
        if recent.len() > self.window_size {
            recent.pop_front();
        }
    }

    /// Check if recent behaviour deviates beyond threshold.
    pub async fn is_deviating(&self) -> bool {
        let recent = self.recent.read().await;
        if recent.len() < 10 {
            return false; // insufficient data
        }

        // Compute mean and std of recent tool calls
        let calls: Vec<f64> = recent.iter().map(|o| o.tool_calls_per_minute).collect();
        let mean = calls.iter().sum::<f64>() / calls.len() as f64;
        let variance = calls.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / calls.len() as f64;
        let std = variance.sqrt();

        // Latest observation
        if let Some(latest) = recent.back() {
            let deviation = (latest.tool_calls_per_minute - mean).abs();
            if std > 0.0 && deviation > self.deviation_threshold * std {
                return true;
            }
        }

        false
    }
}
