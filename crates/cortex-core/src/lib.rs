//! Intellecta Cortex – Sovereign Enterprise Intelligence Hub
//!
//! cortex-core provides the foundational runtime:
//! - Config loading & validation
//! - License feature gating
//! - Main event loop orchestration

pub mod config;
pub mod feature_gate;
pub mod runtime;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Top-level Cortex orchestrator – single entry point.
pub struct CortexRuntime {
    pub config: config::Config,
    pub feature_gate: feature_gate::FeatureGate,
    pub start_time: chrono::DateTime<chrono::Utc>,
    inner: Arc<RwLock<runtime::RuntimeInner>>,
}

impl CortexRuntime {
    /// Bootstrap the entire Cortex platform from a config file.
    pub async fn initialize(config_path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        // Install a basic tracing subscriber until the full one is up.

        let config = config::Config::load(config_path)?;
        let feature_gate = feature_gate::FeatureGate::from_license(&config.license)?;

        info!(
            customer = %config.license.customer,
            plan = %config.license.plan,
            "Cortex initialising"
        );

        let start_time = chrono::Utc::now();
        let inner = Arc::new(RwLock::new(runtime::RuntimeInner::new(&config).await?));

        Ok(Self {
            config,
            feature_gate,
            start_time,
            inner,
        })
    }

    /// Run the main event loop.
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let cortex = Arc::new(self);
        runtime::main_loop(Arc::clone(&cortex)).await
    }

    /// Access the runtime inner state.
    pub async fn inner(&self) -> tokio::sync::RwLockReadGuard<'_, runtime::RuntimeInner> {
        self.inner.read().await
    }

    /// Mutable access to the runtime inner state.
    pub async fn inner_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, runtime::RuntimeInner> {
        self.inner.write().await
    }
}
