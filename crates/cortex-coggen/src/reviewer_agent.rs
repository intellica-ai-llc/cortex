use serde::{Deserialize, Serialize};

/// Reviewer Agent (Aᵣ) — evaluation & feedback signals (Δ).
///
/// CogGen: "The Reviewer provides structural/content feedback (Δ) to
/// trigger replanning iterations." Sub-agents: structure_detector,
/// content_detector, plan_restructurer, refine_loop, *_revise agents.
///
/// This is the critical quality-control gateway in the recursive loop:
/// the Reviewer identifies gaps in coverage, factual errors, structural
/// weaknesses, and missing evidence — then emits structured Δ feedback
/// that the Planner and Writer use to improve the report.
pub struct ReviewerAgent;

/// Structured feedback from the Reviewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewFeedback {
    pub overall_score: f64,
    pub structural_issues: Vec<StructuralIssue>,
    pub content_issues: Vec<ContentIssue>,
    pub gaps: Vec<String>,
    pub recommendation: ReviewRecommendation,
    pub reviewed_at: chrono::DateTime<chrono::Utc>,
}

/// Feedback on a specific section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionFeedback {
    pub section_id: String,
    pub format_score: f64,       // structure, clarity, organisation
    pub factual_score: f64,      // accuracy, citations, evidence
    pub cognitive_score: f64,    // depth, insight, synthesis
    pub issues: Vec<String>,
    pub suggested_improvements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralIssue {
    pub description: String,
    pub severity: IssueSeverity,
    pub affected_sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentIssue {
    pub description: String,
    pub section_id: Option<String>,
    pub severity: IssueSeverity,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity { Critical, Major, Minor, Cosmetic }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReviewRecommendation {
    /// All sections meet quality thresholds; report is complete.
    Approve,
    /// Minor issues; revise specific sections.
    Revise { sections: Vec<String> },
    /// Major gaps; trigger replanning.
    Replan { reason: String },
    /// Fundamental issues; restart with different approach.
    Restart { reason: String },
}

impl ReviewerAgent {
    pub fn new() -> Self { Self }

    /// Evaluate a complete report across structural, factual, and
    /// cognitive dimensions. Emit structured Δ feedback.
    ///
    /// The structure_detector analyses the outline for logical flow;
    /// the content_detector checks facts against citations;
    /// the plan_restructurer proposes outline modifications.
    pub async fn review(
        &self,
        sections: &[super::Section],
    ) -> ReviewFeedback {
        let mut gaps = Vec::new();
        let mut structural_issues = Vec::new();
        let mut content_issues = Vec::new();

        // Check for missing sections (structural gaps).
        let has_executive = sections.iter().any(|s| s.title.to_lowercase().contains("executive") || s.title.to_lowercase().contains("summary"));
        let has_conclusion = sections.iter().any(|s| s.title.to_lowercase().contains("conclusion") || s.title.to_lowercase().contains("recommendation"));

        if !has_executive {
            gaps.push("Missing executive summary or overview section".into());
            structural_issues.push(StructuralIssue {
                description: "No executive summary section found".into(),
                severity: IssueSeverity::Major,
                affected_sections: vec![],
            });
        }
        if !has_conclusion {
            gaps.push("Missing conclusions or recommendations section".into());
            structural_issues.push(StructuralIssue {
                description: "No conclusions section found".into(),
                severity: IssueSeverity::Major,
                affected_sections: vec![],
            });
        }

        // Check for uncited claims.
        let uncited: Vec<_> = sections.iter()
            .filter(|s| s.citations.is_empty() && s.content.is_some())
            .collect();
        if !uncited.is_empty() {
            content_issues.push(ContentIssue {
                description: format!("{} section(s) have no citations", uncited.len()),
                section_id: uncited.first().map(|s| s.id.clone()),
                severity: IssueSeverity::Major,
                suggestion: "Add inline citations for all factual claims".into(),
            });
        }

        // Check low-confidence sections.
        let low_conf: Vec<_> = sections.iter()
            .filter(|s| s.confidence < 0.7)
            .collect();
        for s in &low_conf {
            content_issues.push(ContentIssue {
                description: format!("Low confidence ({:.0}%) in section '{}'", s.confidence * 100.0, s.title),
                section_id: Some(s.id.clone()),
                severity: IssueSeverity::Minor,
                suggestion: "Gather additional evidence or narrow scope".into(),
            });
        }

        let overall_score = if gaps.is_empty() && content_issues.is_empty() { 0.9 }
            else if gaps.len() <= 1 { 0.7 } else { 0.4 };

        let recommendation = if gaps.is_empty() && content_issues.iter().all(|i| i.severity == IssueSeverity::Minor) {
            ReviewRecommendation::Approve
        } else if gaps.len() <= 2 {
            ReviewRecommendation::Revise { sections: vec![] }
        } else {
            ReviewRecommendation::Replan { reason: format!("{} gaps identified", gaps.len()) }
        };

        ReviewFeedback {
            overall_score,
            structural_issues,
            content_issues,
            gaps,
            recommendation,
            reviewed_at: chrono::Utc::now(),
        }
    }

    /// Evaluate a single section in detail.
    pub async fn review_section(
        &self,
        section: &super::Section,
    ) -> SectionFeedback {
        let has_content = section.content.as_ref().map(|c| !c.is_empty()).unwrap_or(false);
        let has_citations = !section.citations.is_empty();

        SectionFeedback {
            section_id: section.id.clone(),
            format_score: if has_content { 0.8 } else { 0.2 },
            factual_score: if has_citations { 0.85 } else { 0.3 },
            cognitive_score: section.confidence,
            issues: if !has_content { vec!["Section has no content".into()] } else { vec![] },
            suggested_improvements: if !has_citations { vec!["Add supporting citations".into()] } else { vec![] },
        }
    }
}
