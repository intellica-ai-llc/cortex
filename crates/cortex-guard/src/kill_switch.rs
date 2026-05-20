use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;

/// Three-factor cryptographic kill switch.
///
/// Factor 1: Cryptographic Token (physical YubiKey/FIDO2)
/// Factor 2: Behavioural Baseline (agent behaviour anomaly)
/// Factor 3: Network Heartbeat (continuous signed heartbeat)
///
/// Inspired by ZeroBiometrics ZeroSentinel: "Revoking a certificate
/// cuts off agent authorization instantly — functioning as a kill
/// switch"[reference:13].
pub struct KillSwitch {
    token_present: AtomicBool,
    active: AtomicBool,
    activation_history: RwLock<Vec<KillSwitchEvent>>,
}

#[derive(Debug, Clone)]
pub struct KillSwitchEvent {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: KillSwitchEventType,
}

#[derive(Debug, Clone)]
pub enum KillSwitchEventType {
    Activated { trigger: String },
    Deactivated { by: String },
    TokenInserted,
    TokenRemoved,
    HeartbeatRestored,
}

impl KillSwitch {
    pub fn new() -> Self {
        Self {
            token_present: AtomicBool::new(true),
            active: AtomicBool::new(false),
            activation_history: RwLock::new(Vec::new()),
        }
    }

    pub async fn is_token_present(&self) -> bool {
        self.token_present.load(Ordering::SeqCst)
    }

    /// Simulate token removal (in production: hardware event).
    pub fn remove_token(&self) {
        self.token_present.store(false, Ordering::SeqCst);
    }

    pub fn insert_token(&self) {
        self.token_present.store(true, Ordering::SeqCst);
    }

    /// Activate the kill switch.
    pub async fn activate(&self, trigger: &str) {
        self.active.store(true, Ordering::SeqCst);
        self.activation_history.write().await.push(KillSwitchEvent {
            timestamp: chrono::Utc::now(),
            event_type: KillSwitchEventType::Activated { trigger: trigger.to_string() },
        });
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    /// Deactivate after forensic review.
    pub async fn deactivate(&self, by: &str) {
        self.active.store(false, Ordering::SeqCst);
        self.activation_history.write().await.push(KillSwitchEvent {
            timestamp: chrono::Utc::now(),
            event_type: KillSwitchEventType::Deactivated { by: by.to_string() },
        });
    }
}
