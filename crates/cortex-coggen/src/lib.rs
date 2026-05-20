//! Cortex CogGen™ — Multi-Agent Recursive Report Fabricator (v6).
//!
//! Based on CogGen (NJUNLP, ACL 2026 Findings): Hierarchical Recursive
//! Architecture with Macro-Cognitive Loop and Micro-Cognitive Cycle.
//! Three-agent architecture — Planner, Writer, Reviewer — generates
//! comprehensive, multimodal research reports with recursive refinement.
//!
//! The Planner decomposes complex research questions into sections and
//! sub-questions, assigning each to a specialised research sub-agent.
//! The Writer drafts each section with inline citations, confidence
//! scores, and source provenance. The Reviewer evaluates each section,
//! identifies gaps, and triggers the Planner to spawn additional
//! research sub-agents for missing information. The process is recursive:
//! the Reviewer's feedback (Δ) loops back to the Planner until all
//! sections meet quality thresholds.

pub mod planner_agent;
pub mod writer_agent;
pub mod reviewer_agent;
pub mod recursive_loop;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top-level CogGen orchestrator.
pub struct CogGenEngine {
    pub planner: Arc<planner_agent::PlannerAgent>,
    pub writer: Arc<writer_agent::WriterAgent>,
    pub reviewer: Arc<reviewer_agent::ReviewerAgent>,
    pub recursive_loop: Arc<recursive_loop::RecursiveLoopController>,
    /// Active report generation sessions.
    sessions: RwLock<std::collections::HashMap<String, ReportSession>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportSession {
    pub session_id: String,
    pub question: String,
    pub sections: Vec<Section>,
    pub status: ReportStatus,
    pub iterations: u32,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub citations: Vec<Citation>,
    pub confidence: f64,
    pub status: SectionStatus,
    pub reviewer_feedback: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Citation {
    pub source_url: Option<String>,
    pub source_text: String,
    pub relevance_score: f64,
    pub verified: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ReportStatus { Planning, Writing, Reviewing, Revising, Complete, Failed }
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SectionStatus { Planned, Researching, Drafted, Reviewed, Approved, NeedsRevision }

impl CogGenEngine {
    pub fn new() -> Self {
        Self {
            planner: Arc::new(planner_agent::PlannerAgent::new()),
            writer: Arc::new(writer_agent::WriterAgent::new()),
            reviewer: Arc::new(reviewer_agent::ReviewerAgent::new()),
            recursive_loop: Arc::new(recursive_loop::RecursiveLoopController::new()),
            sessions: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Generate a complete research report via the recursive pipeline.
    pub async fn generate(&self, question: &str, domain: &str) -> Result<ReportSession, String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = ReportSession {
            session_id: session_id.clone(),
            question: question.to_string(),
            sections: vec![],
            status: ReportStatus::Planning,
            iterations: 0,
            started_at: chrono::Utc::now(),
        };
        self.sessions.write().await.insert(session_id, session);
        self.recursive_loop.run(question, domain).await
    }
}
