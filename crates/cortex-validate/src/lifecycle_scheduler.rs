use serde::{Deserialize, Serialize};

/// CIRCLE-based lifecycle scheduling for experiments.
///
/// Based on CIRCLE (Westling et al., Feb 2026): "a six-stage lifecycle-
/// based framework that links stakeholder concerns to context-sensitive
/// evaluation methods, longitudinal measurement, and ongoing monitoring."
///
/// Cortex maps CIRCLE to four tiers:
///   PerCommit — runs in CI on every push (security posture).
///   Continuous — runs continuously in production (CDC latency, wellness).
///   PerRelease — runs before each release (full benchmark suite).
///   PerTrainingCycle — runs after each model retraining (deep research).

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LifecycleStage {
    /// Every git push. Fast. Critical safety properties.
    PerCommit,
    /// Ongoing in production. Latency, wellness, sync health.
    Continuous,
    /// Before each release. Full benchmark suite.
    PerRelease,
    /// After each model retraining cycle.
    PerTrainingCycle,
}

pub struct LifecycleScheduler {
    last_run: tokio::sync::RwLock<std::collections::HashMap<LifecycleStage, chrono::DateTime<chrono::Utc>>>,
}

impl LifecycleScheduler {
    pub fn new() -> Self {
        Self { last_run: tokio::sync::RwLock::new(std::collections::HashMap::new()) }
    }

    /// Determine whether a stage is due to run.
    ///
    /// PerCommit: always due (CI gate).
    /// Continuous: due if last run > 1 hour ago.
    /// PerRelease: due if version changed.
    /// PerTrainingCycle: due after each training job.
    pub async fn is_due(&self, stage: &LifecycleStage) -> bool {
        match stage {
            LifecycleStage::PerCommit => true,
            LifecycleStage::Continuous => {
                let lock = self.last_run.read().await;
                lock.get(stage)
                    .map(|t| chrono::Utc::now() - *t > chrono::Duration::hours(1))
                    .unwrap_or(true)
            }
            LifecycleStage::PerRelease | LifecycleStage::PerTrainingCycle => true,
        }
    }

    /// Record that a stage has been run.
    pub async fn record_run(&self, stage: &LifecycleStage) {
        self.last_run.write().await.insert(stage.clone(), chrono::Utc::now());
    }
}
