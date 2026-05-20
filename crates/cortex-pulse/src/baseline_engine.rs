use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 30/45/90‑day personal baseline computation.
pub struct BaselineEngine {
    baselines: HashMap<String, PersonalBaseline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalBaseline {
    pub user_id: String,
    pub avg_pulse_score: f64,
    pub std_pulse_score: f64,
    pub days_tracked: u32,
    pub established_at: chrono::DateTime<chrono::Utc>,
}

impl BaselineEngine {
    pub fn new() -> Self { Self { baselines: HashMap::new() } }

    pub fn update(&mut self, user_id: &str, score: f64) {
        let entry = self.baselines.entry(user_id.to_string()).or_insert_with(|| PersonalBaseline {
            user_id: user_id.to_string(),
            avg_pulse_score: 0.0,
            std_pulse_score: 0.0,
            days_tracked: 0,
            established_at: chrono::Utc::now(),
        });
        let n = entry.days_tracked as f64;
        entry.avg_pulse_score = (entry.avg_pulse_score * n + score) / (n + 1.0);
        entry.days_tracked += 1;
    }
}
