use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Personalised daily intelligence brief.
///
/// Like Lofty AI Dashboard’s “Morning Briefing”: a multimodal,
/// voice‑enabled AI summary that gives every user the pulse of
/// their pipeline and their daily agenda. Generated at the start
/// of each user’s day.
pub struct MorningBrief {
    briefs: RwLock<Vec<DailyBrief>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyBrief {
    pub user_id: String,
    pub date: chrono::NaiveDate,
    pub greeting: String,
    pub key_metrics: Vec<MetricSnapshot>,
    pub cross_system_insight: Option<String>,
    pub pending_actions: Vec<String>,
    pub wellness_pulse: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSnapshot {
    pub name: String,
    pub value: f64,
    pub change_pct: f64,     // change from previous period
    pub benchmark: Option<f64>,
}

impl MorningBrief {
    pub fn new() -> Self {
        Self { briefs: RwLock::new(Vec::new()) }
    }

    /// Generate the morning brief for a user.
    pub async fn generate(&self, user_id: &str) -> DailyBrief {
        let brief = DailyBrief {
            user_id: user_id.to_string(),
            date: chrono::Utc::now().date_naive(),
            greeting: "Good morning.".into(),
            key_metrics: vec![],
            cross_system_insight: None,
            pending_actions: vec![],
            wellness_pulse: None,
        };
        self.briefs.write().await.push(brief.clone());
        brief
    }
}
