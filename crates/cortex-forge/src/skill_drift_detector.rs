pub struct SkillDriftDetector {
    consecutive_failures: tokio::sync::Mutex<u32>,
}

impl SkillDriftDetector {
    pub fn new() -> Self { Self { consecutive_failures: tokio::sync::Mutex::new(0) } }
    pub async fn record_failure(&self) {
        *self.consecutive_failures.lock().await += 1;
    }
    pub async fn should_repair(&self) -> bool {
        *self.consecutive_failures.lock().await >= 3
    }
}
