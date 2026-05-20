//! Generates natural‑language compliance and performance feedback tied to the
//! organisation's industry baseline.

use serde::{Deserialize, Serialize};

pub struct FeedbackGenerator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFeedback {
    pub doc_id: String,
    pub summary: String,
    pub detailed_findings: Vec<String>,
    pub recommendations: Vec<String>,
    pub industry_benchmark_comparison: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl FeedbackGenerator {
    pub fn new() -> Self { Self }

    /// Generate natural‑language feedback from cross‑reference results.
    ///
    /// Example output: "PMC: 92% compliant. Below 95% industry benchmark for top‑
    /// performing plants. Recommendation: schedule overdue PMs on assets A‑207,
    /// B‑015, C‑042. NERC CIP‑015‑1: compliant. OSHA 1910.119: 2 minor gaps
    /// identified in mechanical integrity documentation."
    pub fn generate(
        &self,
        cross_ref: &super::benchmark_cross_reference::CrossReferenceResult,
        benchmarks: &super::benchmark_cross_reference::IndustryBenchmarks,
    ) -> ComplianceFeedback {
        let mut findings = Vec::new();
        let mut recommendations = Vec::new();

        for gap in &cross_ref.gaps {
            findings.push(format!("[{}] {}: {}", gap.severity_str(), gap.clause, gap.description));
            recommendations.push(gap.recommendation.clone());
        }

        let comparison = format!(
            "Industry benchmark ({}): PMC target >{:.0}%, reactive work <{:.0}%, \
             OEE >{:.0}%, MTTD <{:.0}min, schedule compliance >{:.0}%, \
             audit readiness {:.0}%.",
            benchmarks.source,
            benchmarks.preventive_maintenance_compliance,
            benchmarks.reactive_work_pct_max,
            benchmarks.oee_target,
            benchmarks.mttd_minutes_max,
            benchmarks.schedule_compliance_pct,
            benchmarks.audit_readiness_pct,
        );

        let summary = if cross_ref.compliance_score >= 95.0 {
            format!(
                "Document is {:.0}% compliant across {} frameworks. \
                 No critical gaps identified.",
                cross_ref.compliance_score,
                cross_ref.frameworks_checked.len(),
            )
        } else if cross_ref.compliance_score >= 80.0 {
            format!(
                "Document is {:.0}% compliant. {} minor gaps found. \
                 See recommendations below.",
                cross_ref.compliance_score,
                cross_ref.gaps.len(),
            )
        } else {
            format!(
                "Document is {:.0}% compliant. {} gaps require attention \
                 before next audit.",
                cross_ref.compliance_score,
                cross_ref.gaps.len(),
            )
        };

        ComplianceFeedback {
            doc_id: cross_ref.doc_id.clone(),
            summary,
            detailed_findings: findings,
            recommendations,
            industry_benchmark_comparison: comparison,
            generated_at: chrono::Utc::now(),
        }
    }
}

impl super::benchmark_cross_reference::GapSeverity {
    fn severity_str(&self) -> &str {
        match self {
            Self::Critical => "CRITICAL",
            Self::Major => "MAJOR",
            Self::Minor => "MINOR",
            Self::Advisory => "ADVISORY",
        }
    }
}
