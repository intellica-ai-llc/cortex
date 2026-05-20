use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Proactive alerts that pull users into Cortex instead of requiring
/// them to remember to check it.
///
/// Part of the “addictive” UX architecture: alerts are role‑specific,
/// learned from behaviour, and delivered across devices.
pub struct NotificationManager {
    /// Subscriptions per user per channel.
    subscriptions: RwLock<HashMap<String, Vec<NotificationChannel>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub body: String,
    pub severity: NotificationSeverity,
    pub action: Option<NotificationAction>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationAction {
    pub label: String,
    pub action_type: ActionType,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    OpenPanel,
    ExecuteSkill,
    ViewReport,
    ApproveRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    InApp,
    Email,
    Slack,
    Teams,
    Push,
    SMS,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self { subscriptions: RwLock::new(HashMap::new()) }
    }

    /// Send a notification to a user through all active channels.
    pub async fn notify(&self, notification: Notification) {
        tracing::info!(
            user = %notification.user_id,
            title = %notification.title,
            "Sending notification"
        );
        // In production: dispatch via configured channels.
    }

    /// Register a channel preference for a user.
    pub async fn set_channels(&self, user_id: &str, channels: Vec<NotificationChannel>) {
        self.subscriptions.write().await.insert(user_id.to_string(), channels);
    }
}
