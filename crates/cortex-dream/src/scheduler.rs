use std::sync::atomic::{AtomicBool, Ordering};
use chrono::Utc;

/// Determines when the dream cycle should run.
pub struct DreamCycleScheduler {
    last_dream: tokio::sync::Mutex<chrono::DateTime<Utc>>,
    min_interval_seconds: i64,
    is_running: AtomicBool,
}

impl DreamCycleScheduler {
    pub fn new() -> Self {
        Self {
            last_dream: tokio::sync::Mutex::new(Utc::now()),
            min_interval_seconds: 3600 * 6, // at most once every 6 hours
            is_running: AtomicBool::new(false),
        }
    }

    /// Returns true if enough time has passed since the last dream cycle.
    pub fn should_dream(&self) -> bool {
        if self.is_running.load(Ordering::SeqCst) {
            return false;
        }
        let last = self.last_dream.try_lock().unwrap();
        let elapsed = Utc::now() - *last;
        elapsed.num_seconds() >= self.min_interval_seconds
    }

    /// Mark the dream cycle as started and update the last run timestamp.
    pub async fn mark_started(&self) {
        self.is_running.store(true, Ordering::SeqCst);
        *self.last_dream.lock().await = Utc::now();
    }

    /// Mark the dream cycle as completed.
    pub fn mark_completed(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}
