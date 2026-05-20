use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// UX Preference Store — per‑user interface preferences.
pub struct UXPreferenceStore {
    preferences: RwLock<HashMap<String, UserUXProfile>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUXProfile {
    pub user_id: String,
    pub preferred_chart_type: String,
    pub density: String,
    pub notification_frequency: String,
    pub learned_panels: Vec<String>,
}

impl UXPreferenceStore {
    pub fn new() -> Self {
        Self { preferences: RwLock::new(HashMap::new()) }
    }

    pub async fn save(&self, profile: UserUXProfile) {
        self.preferences.write().await.insert(profile.user_id.clone(), profile);
    }

    pub async fn load(&self, user_id: &str) -> Option<UserUXProfile> {
        self.preferences.read().await.get(user_id).cloned()
    }
}
