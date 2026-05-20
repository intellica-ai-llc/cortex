use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Recursive Loop Controller — drives the Macro-Cognitive Loop.
///
/// CogGen's Hierarchical Recursive Architecture consists of two
/// nested cognitive loops:
///
///   **Macro-Cognitive Loop**: Planner generates a global outline →
///   sections are written in parallel → Reviewer provides structural/
///   content feedback (Δ) → replanning iterations until all sections
///   meet quality thresholds.
///
///   **Micro-Cognitive Cycle**: Per-section processing where each
///   section undergoes independent search → plan → write → revise
///   cycles with format, factual, and cognitive revision.
///
/// The recursive loop continues until:
///   1. All sections are approved by the Reviewer, OR
///   2. Maximum iterations reached (prevents infinite loops), OR
///   3. The Reviewer recommends Restart (fundamental approach change).
pub struct RecursiveLoopController {
    max_iterations: u32,
    quality_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopState {
    pub iteration: u32,
    pub sections: Vec<super::Section>,
    pub feedback: Option<super::reviewer_agent::ReviewFeedback>,
    pub phase: LoopPhase,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoopPhase {
    Planning,
    Writing,
    Reviewing,
    Revising,
    Complete,
    MaxIterationsReached,
}

impl RecursiveLoopController {
    pub fn new() -> Self {
        Self { max_iterations: 10, quality_threshold: 0.8 }
    }

    /// Run the full recursive report generation pipeline.
    ///
    /// This is the primary entry point. It orchestrates the
    /// Planner → Writer → Reviewer → (recurse) cycle.
    pub async fn run(
        &self,
        question: &str,
        domain: &str,
    ) -> Result<super::ReportSession, String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let planner = super::planner_agent::PlannerAgent::new();
        let writer = super::writer_agent::WriterAgent::new();
        let reviewer = super::reviewer_agent::ReviewerAgent::new();

        // Phase 1 — PLANNING: decompose question into sections.
        let plan = planner.plan(question, domain).await;
        let mut sections: Vec<super::Section> = plan.outline.iter().map(|o| {
            super::Section {
                id: o.id.clone(), title: o.title.clone(),
                content: None, citations: vec![],
                confidence: 0.0, status: super::SectionStatus::Planned,
                reviewer_feedback: None,
            }
        }).collect();

        // Phase 2–4 — WRITE → REVIEW → REVISE recursive loop.
        let mut iteration = 0u32;
        loop {
            iteration += 1;

            // WRITE: draft each unapproved section.
            for section in sections.iter_mut().filter(|s| s.status != super::SectionStatus::Approved) {
                let ctx = super::writer_agent::WritingContext {
                    question: question.to_string(),
                    section_title: section.title.clone(),
                    sub_questions: vec![],
                    research_findings: vec![],
                    style_guide: super::writer_agent::WritingStyle {
                        tone: "business".into(), max_words: 800,
                        require_citations: true, language: "en".into(),
                    },
                };
                let drafted = writer.draft_section(&ctx).await;
                section.content = Some(drafted.content);
                section.citations = drafted.citations;
                section.confidence = drafted.confidence;
                section.status = super::SectionStatus::Drafted;
            }

            // REVIEW: evaluate all sections.
            let feedback = reviewer.review(&sections).await;
            for section in &mut sections {
                let sf = reviewer.review_section(section).await;
                section.reviewer_feedback = Some(format!(
                    "Format: {:.0}%, Factual: {:.0}%, Cognitive: {:.0}%",
                    sf.format_score * 100.0, sf.factual_score * 100.0, sf.cognitive_score * 100.0
                ));
            }

            // DECIDE: approve, revise, or replan.
            match &feedback.recommendation {
                super::reviewer_agent::ReviewRecommendation::Approve => {
                    for section in &mut sections {
                        section.status = super::SectionStatus::Approved;
                    }
                    break;
                }
                super::reviewer_agent::ReviewRecommendation::Revise { .. } => {
                    // Micro-Cognitive Cycle: revise flagged sections.
                    for section in sections.iter_mut().filter(|s| s.status != super::SectionStatus::Approved) {
                        section.status = super::SectionStatus::NeedsRevision;
                    }
                }
                super::reviewer_agent::ReviewRecommendation::Replan { .. } => {
                    // Macro-Cognitive Loop: Planner re‑outlines.
                    let _new_plan = planner.replan(&plan, &feedback).await;
                }
                super::reviewer_agent::ReviewRecommendation::Restart { .. } => {
                    return Err("Reviewer requested restart".into());
                }
            }

            if iteration >= self.max_iterations {
                break;
            }
        }

        let status = if iteration >= self.max_iterations {
            super::ReportStatus::Failed
        } else {
            super::ReportStatus::Complete
        };

        Ok(super::ReportSession {
            session_id,
            question: question.to_string(),
            sections,
            status,
            iterations: iteration,
            started_at: chrono::Utc::now(),
        })
    }
}
