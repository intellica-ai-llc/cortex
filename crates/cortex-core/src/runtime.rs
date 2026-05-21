use crate::{config::Config, CortexRuntime};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::info;

pub struct RuntimeInner {
    pub heartbeat_count: u64,
    pub active_sessions: u64,
}

impl RuntimeInner {
    pub async fn new(_config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { heartbeat_count: 0, active_sessions: 0 })
    }
}

pub async fn main_loop(cortex: Arc<CortexRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Cortex main loop starting");
    loop {
        {
            let mut inner = cortex.inner.write().await;
            inner.heartbeat_count += 1;
            if inner.heartbeat_count % 60 == 0 {
                info!(heartbeat = inner.heartbeat_count, sessions = inner.active_sessions, "Cortex heartbeat");
            }
        }
        sleep(Duration::from_millis(100)).await;
    }
}
