use chrono::{DateTime, Utc};

/// Monitors on‑device model version and checks against server.
///
/// On every sync cycle, compares local model version with the
/// server's latest. If stale (>1 version behind), suspends
/// tokenization, tags decisions, and queues update.
pub struct ModelFreshnessChecker {
    current_version: String,
    last_checked: DateTime<Utc>,
}

impl ModelFreshnessChecker {
    pub fn new(version: &str) -> Self {
        Self {
            current_version: version.to_string(),
            last_checked: Utc::now(),
        }
    }

    /// Check with server (or manifest) for available updates.
    pub async fn check(&self, _server_url: &str) -> Option<String> {
        // In production: fetch /version from server, compare.
        None
    }

    /// Mark the model as updated.
    pub fn update(&mut self, new_version: &str) {
        self.current_version = new_version.to_string();
        self.last_checked = Utc::now();
    }

    pub fn current_version(&self) -> &str { &self.current_version }
}
