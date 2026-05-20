use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Adoption Bridge Sequencer — crossing Moore’s Chasm.
///
/// The Octalysis Voluntary Adoption Cascade shows that voluntary
/// migration stalls at ~16% (the Moore’s Chasm boundary between
/// Early Adopters and Early Majority) without an explicit bridge
/// of social proof, simplified onboarding, and reduced risk
/// perception.
///
/// This sequencer detects when a user’s absorbed workflows cross
/// the 16% threshold and triggers the bridge events.
pub struct AdoptionBridgeSequencer {
    /// Per‑user adoption metrics.
    metrics: RwLock<Vec<AdoptionMetric>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionMetric {
    pub user_id: String,
    pub absorbed_workflows: u64,
    pub total_workflows: u64,
    pub absorption_pct: f64,
    pub chasm_crossed: bool,
    pub bridge_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeEvent {
    pub user_id: String,
    pub event_type: BridgeEventType,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeEventType {
    EarlyAdopterSocialProof,
    TimeSavedSummary,
    SimplifiedOnboarding,
    RiskReductionDemo,
}

impl AdoptionBridgeSequencer {
    pub fn new() -> Self {
        Self { metrics: RwLock::new(Vec::new()) }
    }

    /// Update a user’s absorption percentage and check chasm.
    pub async fn update_progress(&self, user_id: &str, absorbed: u64, total: u64) -> Option<BridgeEvent> {
        let pct = if total == 0 { 0.0 } else { absorbed as f64 / total as f64 };
        let mut metrics = self.metrics.write().await;
        let metric = metrics.iter_mut().find(|m| m.user_id == user_id);
        let crossed = pct >= 0.16 && !metric.as_ref().map(|m| m.chasm_crossed).unwrap_or(false);

        if let Some(m) = metric {
            m.absorbed_workflows = absorbed;
            m.total_workflows = total;
            m.absorption_pct = pct;
            if crossed {
                m.chasm_crossed = true;
                m.bridge_triggered_at = Some(chrono::Utc::now());
            }
        } else {
            metrics.push(AdoptionMetric {
                user_id: user_id.to_string(),
                absorbed_workflows: absorbed,
                total_workflows: total,
                absorption_pct: pct,
                chasm_crossed: crossed,
                bridge_triggered_at: if crossed { Some(chrono::Utc::now()) } else { None },
            });
        }

        if crossed {
            Some(BridgeEvent {
                user_id: user_id.to_string(),
                event_type: BridgeEventType::TimeSavedSummary,
                message: format!(
                    "You’ve saved {} minutes this week by using Cortex instead of switching between legacy apps. \
                     {} colleagues have already made the switch.",
                    absorbed * 5, // estimate
                    self.count_early_adopters().await
                ),
                timestamp: chrono::Utc::now(),
            })
        } else {
            None
        }
    }

    /// Count users who have crossed the chasm (for social proof).
    async fn count_early_adopters(&self) -> usize {
        self.metrics.read().await.iter().filter(|m| m.chasm_crossed).count()
    }
}
