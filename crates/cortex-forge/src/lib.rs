//! Cortex Forge™ — Self‑Programming Skill Engine (v7).
//!
//! Auto‑generates, curates, publishes, and deprecates agent skills
//! from observed workflows, with RL bootstrapping and drift detection.

pub mod skill_synthesis;
pub mod curator;
pub mod marketplace_federated;
pub mod auto_deprecation;
pub mod skill_drift_detector;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ForgeEngine {
    pub synthesis: Arc<skill_synthesis::SkillSynthesisEngine>,
    pub curator: Arc<curator::Curator>,
    pub marketplace: Arc<marketplace_federated::FederatedMarketplace>,
    pub deprecation: Arc<auto_deprecation::AutoDeprecation>,
    pub drift_detector: Arc<skill_drift_detector::SkillDriftDetector>,
    pub skill_library: RwLock<std::collections::HashMap<String, skill_synthesis::ForgeSkill>>,
}

impl ForgeEngine {
    pub fn new() -> Self {
        Self {
            synthesis: Arc::new(skill_synthesis::SkillSynthesisEngine::new()),
            curator: Arc::new(curator::Curator::new()),
            marketplace: Arc::new(marketplace_federated::FederatedMarketplace::new()),
            deprecation: Arc::new(auto_deprecation::AutoDeprecation::new()),
            drift_detector: Arc::new(skill_drift_detector::SkillDriftDetector::new()),
            skill_library: RwLock::new(std::collections::HashMap::new()),
        }
    }
}
