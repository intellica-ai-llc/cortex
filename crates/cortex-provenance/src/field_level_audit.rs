use std::sync::Arc;
use tokio::sync::RwLock;

/// Per‑field access and change logging.
pub struct FieldLevelAuditTrail {
    events: RwLock<Vec<FieldAuditEvent>>,
}

#[derive(Debug, Clone)]
pub struct FieldAuditEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
    pub field_path: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

impl FieldLevelAuditTrail {
    pub fn new() -> Self {
        Self { events: RwLock::new(Vec::new()) }
    }

    pub async fn log(&self, event: FieldAuditEvent) {
        self.events.write().await.push(event);
    }

    pub async fn query(&self, field: &str) -> Vec<FieldAuditEvent> {
        self.events.read().await.iter()
            .filter(|e| e.field_path == field)
            .cloned()
            .collect()
    }
}
