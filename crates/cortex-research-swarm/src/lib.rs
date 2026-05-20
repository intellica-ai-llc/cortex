//! Cortex Research Swarm™ — Collaborative Multi-Agent Research (v6).
//!
//! Based on the AI Scientific Community model (Braga-Neto, arXiv:2603.21344,
//! Mar 2026): "agentic swarms of virtual labs where each particle in the
//! swarm represents a complete virtual laboratory instance, enabling
//! collective scientific exploration." ZetaSwarm (Lantern Pharma, May 7,
//! 2026): "a coordinated network of specialist AI agents that operate in
//! parallel on scientific sub-problems and converge on a synthesized answer
//! through a coordinator-and-reviewer architecture."
//!
//! Each research agent operates on a sub-question with its own
//! IterResearch workspace. The Swarm Leader decomposes the main question,
//! assigns sub-questions, and orchestrates the campaign. The Synthesiser
//! resolves conflicts and merges findings into a unified report.

pub mod swarm_leader;
pub mod research_subagent;
pub mod synthesiser;
pub mod consensus_protocol;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ResearchSwarm {
    pub leader: Arc<swarm_leader::SwarmLeader>,
    pub synthesiser: Arc<synthesiser::Synthesiser>,
    pub consensus: Arc<consensus_protocol::SwarmConsensusProtocol>,
    /// Active sub-agents indexed by ID.
    agents: RwLock<std::collections::HashMap<String, research_subagent::ResearchSubAgent>>,
    /// Active campaigns.
    campaigns: RwLock<std::collections::HashMap<String, SwarmCampaign>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SwarmCampaign {
    pub campaign_id: String,
    pub main_question: String,
    pub sub_questions: Vec<SubQuestionAssignment>,
    pub status: CampaignStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubQuestionAssignment {
    pub id: String,
    pub question: String,
    pub assigned_agent_id: Option<String>,
    pub domain: String,
    pub status: SubQuestionStatus,
    pub findings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum CampaignStatus { Planning, Researching, Synthesising, Complete, Failed }
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SubQuestionStatus { Pending, Assigned, Researching, Complete, Failed }

impl ResearchSwarm {
    pub fn new() -> Self {
        Self {
            leader: Arc::new(swarm_leader::SwarmLeader::new()),
            synthesiser: Arc::new(synthesiser::Synthesiser::new()),
            consensus: Arc::new(consensus_protocol::SwarmConsensusProtocol::new()),
            agents: RwLock::new(std::collections::HashMap::new()),
            campaigns: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Spawn a new research campaign.
    pub async fn campaign(&self, question: &str) -> Result<SwarmCampaign, String> {
        let campaign_id = uuid::Uuid::new_v4().to_string();
        let sub_questions = self.leader.decompose(question).await;
        let campaign = SwarmCampaign {
            campaign_id: campaign_id.clone(),
            main_question: question.to_string(),
            sub_questions,
            status: CampaignStatus::Planning,
            started_at: chrono::Utc::now(),
        };
        self.campaigns.write().await.insert(campaign_id, campaign);
        Ok(campaign)
    }

    /// Synthesise findings from all sub-agents.
    pub async fn synthesise(&self, campaign_id: &str) -> Result<serde_json::Value, String> {
        let campaigns = self.campaigns.read().await;
        let campaign = campaigns.get(campaign_id).ok_or("Campaign not found")?;
        self.synthesiser.synthesise(&campaign.sub_questions).await
    }
}
