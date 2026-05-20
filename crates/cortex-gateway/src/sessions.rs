use std::collections::HashMap;
use tokio::sync::RwLock;

/// Initiative‑scoped session management.
pub struct SessionManager {
    sessions: RwLock<HashMap<String, Session>>,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self { sessions: RwLock::new(HashMap::new()) }
    }

    pub async fn create(&self, user_id: Option<String>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.sessions.write().await.insert(id.clone(), Session {
            id: id.clone(),
            user_id,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(8),
        });
        id
    }

    pub async fn validate(&self, id: &str) -> bool {
        if let Some(session) = self.sessions.read().await.get(id) {
            chrono::Utc::now() < session.expires_at
        } else {
            false
        }
    }
}
