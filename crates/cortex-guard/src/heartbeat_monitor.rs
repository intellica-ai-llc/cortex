use std::sync::atomic::{AtomicI64, Ordering};
use chrono::Utc;

/// Continuous signed heartbeat monitor.
///
/// Factor 3 of the CortexGuard kill switch. If the heartbeat is
/// lost for more than 30 seconds, all agents enter safe-park mode.
pub struct HeartbeatMonitor {
    last_heartbeat: AtomicI64, // Unix timestamp
    timeout_seconds: i64,
}

impl HeartbeatMonitor {
    pub fn new() -> Self {
        Self {
            last_heartbeat: AtomicI64::new(Utc::now().timestamp()),
            timeout_seconds: 30,
        }
    }

    /// Record a heartbeat (called by the monitoring station).
    pub fn heartbeat(&self) {
        self.last_heartbeat.store(Utc::now().timestamp(), Ordering::SeqCst);
    }

    /// Check if the heartbeat is still alive.
    pub async fn is_alive(&self) -> bool {
        let last = self.last_heartbeat.load(Ordering::SeqCst);
        let now = Utc::now().timestamp();
        (now - last) < self.timeout_seconds
    }

    /// Seconds since last heartbeat.
    pub fn seconds_since_last(&self) -> i64 {
        Utc::now().timestamp() - self.last_heartbeat.load(Ordering::SeqCst)
    }
}
