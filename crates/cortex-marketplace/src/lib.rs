//! Cortex Marketplace™ — Enterprise Agent Economy (v6/v7).
//!
//! Enables opt-in sharing of anonymised research trajectories (DP ε=1),
//! publishing of domain-specific agent skills, and outcome-based billing
//! (per report, per filing, per brief). Based on Nevermined AI Agent
//! Card Payments and Perplexity's shift to consumption pricing.

pub mod trajectory_sharing;
pub mod skill_publisher;
pub mod outcome_billing;
pub mod credit_system;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MarketplaceEngine {
    pub trajectory_sharing: Arc<trajectory_sharing::TrajectorySharingProtocol>,
    pub skill_publisher: Arc<skill_publisher::SkillPublisher>,
    pub billing: Arc<outcome_billing::OutcomeBillingEngine>,
    pub credits: Arc<credit_system::CreditSystem>,
}

impl MarketplaceEngine {
    pub fn new() -> Self {
        Self {
            trajectory_sharing: Arc::new(trajectory_sharing::TrajectorySharingProtocol::new()),
            skill_publisher: Arc::new(skill_publisher::SkillPublisher::new()),
            billing: Arc::new(outcome_billing::OutcomeBillingEngine::new()),
            credits: Arc::new(credit_system::CreditSystem::new()),
        }
    }
}
