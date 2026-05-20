use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Cross‑Device Session Manager (v3).
///
/// Preserves context when a user switches between desktop,
/// laptop, and mobile. A query started on the desktop is
/// waiting on the mobile dashboard with full context.
pub struct CrossDeviceSessionManager {
    /// Active sessions per user, indexed by device type.
    sessions: RwLock<HashMap<String, Vec<DeviceSession>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSession {
    pub device_id: String,
    pub device_type: DeviceType,
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub context: Option<serde_json::Value>, // serialised dashboard state
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType { Desktop, Laptop, Mobile, Tablet }

impl CrossDeviceSessionManager {
    pub fn new() -> Self {
        Self { sessions: RwLock::new(HashMap::new()) }
    }

    /// Update the context for a user’s device.
    pub async fn update_context(&self, user_id: &str, device: DeviceSession) {
        let mut map = self.sessions.write().await;
        let devices = map.entry(user_id.to_string()).or_default();
        if let Some(existing) = devices.iter_mut().find(|d| d.device_id == device.device_id) {
            *existing = device;
        } else {
            devices.push(device);
        }
    }

    /// Retrieve the latest context for a user from any device.
    pub async fn get_latest_context(&self, user_id: &str) -> Option<serde_json::Value> {
        let map = self.sessions.read().await;
        let devices = map.get(user_id)?;
        devices.iter()
            .max_by_key(|d| d.last_active)
            .and_then(|d| d.context.clone())
    }
}
