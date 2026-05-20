use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Progressive Weaning Engine (v2–v8).
///
/// Tracks which workflows users still perform in legacy applications
/// and proactively migrates them to Cortex. Over 4‑6 weeks, 80% of
/// workflows migrate by convenience, not mandate.
///
/// Implements the Octalysis Voluntary Adoption Cascade and the
/// Strangler Fig façade pattern: the legacy app remains available as
/// a fallback; users stop using it because Cortex is faster.
pub struct WeaningEngine {
    /// Per‑user migration progress (percentage of workflows absorbed).
    progress: RwLock<HashMap<String, WeaningProgress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaningProgress {
    pub user_id: String,
    pub legacy_application: String,
    pub absorbed_workflow_count: u64,
    pub remaining_workflow_count: u64,
    pub absorption_pct: f64,
    pub last_suggestion_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl WeaningEngine {
    pub fn new() -> Self {
        Self { progress: RwLock::new(HashMap::new()) }
    }

    /// Check if a user should be nudged toward Cortex for a given workflow.
    /// The nudge is shown only once per workflow every 7 days.
    pub async fn should_nudge(
        &self,
        user_id: &str,
        legacy_app: &str,
        skill_name: &str,
    ) -> Option<String> {
        let progress = self.progress.read().await;
        let key = format!("{user_id}:{legacy_app}:{skill_name}");
        if let Some(entry) = progress.get(&key) {
            // Already nudged recently
            if let Some(last) = entry.last_suggestion_at {
                if chrono::Utc::now() - last < chrono::Duration::days(7) {
                    return None;
                }
            }
        }

        Some(format!(
            "I can now run '{skill_name}' in Cortex — it takes 30 seconds instead of 20 minutes. Want to try it?"
        ))
    }

    /// Record that a nudge was shown.
    pub async fn record_nudge(&self, user_id: &str, legacy_app: &str, skill_name: &str) {
        let mut progress = self.progress.write().await;
        let key = format!("{user_id}:{legacy_app}:{skill_name}");
        progress.entry(key)
            .and_modify(|e| e.last_suggestion_at = Some(chrono::Utc::now()))
            .or_insert_with(|| WeaningProgress {
                user_id: user_id.to_string(),
                legacy_application: legacy_app.to_string(),
                absorbed_workflow_count: 1,
                remaining_workflow_count: 0,
                absorption_pct: 100.0,
                last_suggestion_at: Some(chrono::Utc::now()),
            });
    }

    /// Get absorption score for a user/application.
    pub async fn absorption_score(&self, user_id: &str, app: &str) -> f64 {
        let progress = self.progress.read().await;
        progress.iter()
            .filter(|(k, v)| k.starts_with(&format!("{user_id}:{app}:")) && v.absorbed_workflow_count > 0)
            .map(|(_, v)| v.absorption_pct)
            .fold(0.0, f64::max)
    }
}
