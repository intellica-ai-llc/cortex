use chrono::{DateTime, Utc};

/// Ebbinghaus forgetting curve with reinforcement.
pub struct DecayManager {
    decay_rate: f64,  // λ in exponential decay
}

impl DecayManager {
    pub fn new() -> Self {
        Self { decay_rate: 0.05 }  // ~half‑life of ~14 days
    }

    /// Compute decayed importance of a trace.
    pub fn decayed_importance(
        &self,
        initial_importance: f64,
        last_access: DateTime<Utc>,
        reinforcement_count: u32,
    ) -> f64 {
        let age_days = (Utc::now() - last_access).num_hours() as f64 / 24.0;
        let decay = (-self.decay_rate * age_days).exp();
        let reinforcement = 1.0 + (reinforcement_count as f64 * 0.1);
        (initial_importance * decay * reinforcement).min(1.0)
    }
}
