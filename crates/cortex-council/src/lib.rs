//! Cortex AgentCouncil — Organisational AI Workforce.
//!
//! Based on OMC (arXiv:2604.22446): agents are Talents with portable
//! identities, recruited through a Talent Market, and orchestrated
//! via Explore-Execute-Review (E²R) tree search.
//!
//! OMC achieved 84.67% on PRDBench, surpassing SOTA by 15.48 pp.
//! The 8-agent structure is inherited from Tether Codex v2.

pub mod talent;
pub mod talent_market;
pub mod orchestrator;
pub mod handoff;
pub mod state_manager;
pub mod agents;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Top-level council orchestrator.
pub struct AgentCouncil {
    /// Active talented agents indexed by role.
    pub talents: RwLock<HashMap<String, talent::Talent>>,
    /// Talent market for recruitment.
    pub market: talent_market::TalentMarket,
    /// E²R tree-search orchestrator.
    pub orchestrator: orchestrator::Orchestrator,
    /// Formal delegation protocol.
    pub handoff_manager: handoff::HandoffManager,
    /// Persistent state manager.
    pub state_manager: state_manager::StateManager,
}

impl AgentCouncil {
    pub fn new() -> Self {
        Self {
            talents: RwLock::new(HashMap::new()),
            market: talent_market::TalentMarket::new(),
            orchestrator: orchestrator::Orchestrator::new(),
            handoff_manager: handoff::HandoffManager::new(),
            state_manager: state_manager::StateManager::new(),
        }
    }

    /// Bootstrap the eight core specialist agents (Tether Codex v2).
    pub async fn bootstrap_core_agents(&self) -> Result<(), CouncilError> {
        let core_definitions = vec![
            ("mae", "Master Architect Essence", "Strategic planning, architecture design, initiative decomposition"),
            ("mi",  "Master Innovator",       "Creative problem-solving, novel approaches, R&D exploration"),
            ("pca","Platform Compute Agent",  "Infrastructure provisioning, scaling, resource optimisation"),
            ("db", "Database Expert",         "Schema design, query optimisation, data integrity"),
            ("mm", "Master Marketer",         "Market analysis, competitive intelligence, positioning"),
            ("bug","Debugging Agent",         "Root-cause analysis, error tracing, fix verification"),
            ("qc", "Quality Control Agent",   "Output validation, compliance checks, accuracy verification"),
            ("mnt","Maintenance Master",      "System health, updates, deprecation management"),
        ];

        for (role, name, desc) in core_definitions {
            let t = talent::Talent::new(role, name, desc);
            self.talents.write().await.insert(role.to_string(), t);
        }

        tracing::info!("Bootstrapped 8 core agents (Tether Codex v2)");
        Ok(())
    }

    /// Recruit a specialist agent from the talent market.
    pub async fn recruit(&self, role: &str, required_skills: &[String]) -> Result<talent::Talent, CouncilError> {
        self.market.recruit(role, required_skills).await
    }

    /// Execute a mission via E²R tree search.
    pub async fn execute_mission(
        &self,
        mission: orchestrator::Mission,
    ) -> Result<orchestrator::MissionResult, CouncilError> {
        let talents = self.talents.read().await;
        self.orchestrator.execute(&mission, &talents).await
    }

    /// Formal handoff with context preservation (Tether).
    pub async fn delegate(
        &self,
        from: &str,
        to: &str,
        task: handoff::HandoffTask,
    ) -> Result<handoff::HandoffResult, CouncilError> {
        self.handoff_manager.delegate(from, to, task).await
    }

    /// Get a talent by role name.
    pub async fn get_talent(&self, role: &str) -> Option<talent::Talent> {
        self.talents.read().await.get(role).cloned()
    }

    /// List all active talents.
    pub async fn list_talents(&self) -> Vec<talent::Talent> {
        self.talents.read().await.values().cloned().collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CouncilError {
    #[error("Talent not found: {0}")]
    TalentNotFound(String),
    #[error("Mission execution failed: {0}")]
    MissionFailed(String),
    #[error("Handoff failed: {0}")]
    HandoffFailed(String),
    #[error("Recruitment failed: {0}")]
    RecruitmentFailed(String),
}
