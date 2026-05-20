use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Write Approval Gate — HITL gateway for regulated industries.
///
/// Based on the DZone Commit Boundary design pattern (March 2026):
/// "Deploying Human‑in‑the‑Loop as a universal mandate requiring
/// approval for every agent action proves ineffective in
/// operational environments. The Commit Boundary demarcates the
/// transition from advisory output to executable action."
///
/// Architecture: Agent → Policy Gate → Human Review → Executor.
/// Every state‑modifying operation is:
///   1. Typed and validated against a fixed schema
///   2. Scored and classified by risk tier
///   3. Submitted for human evaluation when risk thresholds exceed
///   4. Processed exclusively by an execution service operating
///      under least‑privilege principles
///   5. Persisted in an immutable log
///
/// For NERC‑CIP (energy), SOX (financial services), and EU AI Act
/// Article 12 (all sectors), human approval on state mutation is
/// legally required — not optional.
pub struct WriteApprovalGate {
    /// Risk thresholds per field category.
    risk_thresholds: RwLock<HashMap<FieldCategory, f64>>,
    /// Pending approval requests.
    pending: RwLock<Vec<ApprovalRequest>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldCategory {
    Operational,     // work orders, maintenance logs
    Financial,       // invoices, purchase orders
    Personnel,       // HR records
    Regulatory,      // compliance filings, audit records
    SensitivePII,    // personal data
    Infrastructure,  // SCADA, network config
}

impl FieldCategory {
    /// Default risk threshold for each category.
    /// Below threshold, auto‑approve; above, HITL required.
    pub fn default_threshold(&self) -> f64 {
        match self {
            Self::Operational => 0.6,
            Self::Financial => 0.3,
            Self::Personnel => 0.3,
            Self::Regulatory => 0.2,
            Self::SensitivePII => 0.1,
            Self::Infrastructure => 0.1,
        }
    }
}

/// A write request submitted for approval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteRequest {
    pub id: String,
    pub source: String,
    pub table: String,
    pub primary_key: String,
    pub column: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub agent_id: Option<String>,
    pub field_category: FieldCategory,
    pub risk_score: f64,
    pub justification: Option<String>,
}

/// An approval decision on a write.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub write: WriteRequest,
    pub status: ApprovalStatus,
    pub requested_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub approver: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalStatus {
    /// Automatically approved (risk below threshold).
    AutoApproved,
    /// Pending human review.
    PendingReview,
    /// Approved.
    Approved,
    /// Denied.
    Denied,
}

impl WriteApprovalGate {
    pub fn new() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert(FieldCategory::Operational, 0.6);
        thresholds.insert(FieldCategory::Financial, 0.3);
        thresholds.insert(FieldCategory::Personnel, 0.3);
        thresholds.insert(FieldCategory::Regulatory, 0.2);
        thresholds.insert(FieldCategory::SensitivePII, 0.1);
        thresholds.insert(FieldCategory::Infrastructure, 0.1);
        Self { risk_thresholds: RwLock::new(thresholds), pending: RwLock::new(Vec::new()) }
    }

    /// Gate a write request. Returns the approval decision.
    ///
    /// If risk_score is below the category threshold, auto‑approve.
    /// Otherwise, queue for human review via the CryptoHITL module.
    pub async fn gate(&self, write: WriteRequest) -> ApprovalRequest {
        let threshold = self.risk_thresholds.read().await
            .get(&write.field_category)
            .copied()
            .unwrap_or(0.5);

        let status = if write.risk_score <= threshold {
            ApprovalStatus::AutoApproved
        } else {
            ApprovalStatus::PendingReview
        };

        let approval = ApprovalRequest {
            write,
            status: status.clone(),
            requested_at: chrono::Utc::now(),
            resolved_at: if status == ApprovalStatus::AutoApproved { Some(chrono::Utc::now()) } else { None },
            approver: None,
            reason: if status == ApprovalStatus::AutoApproved { Some("Risk below threshold".into()) } else { None },
        };

        if status == ApprovalStatus::PendingReview {
            self.pending.write().await.push(approval.clone());
        }

        approval
    }

    /// Record a human approval decision.
    pub async fn approve(&self, request_id: &str, approver: &str, reason: &str) -> Option<ApprovalRequest> {
        let mut pending = self.pending.write().await;
        if let Some(req) = pending.iter_mut().find(|r| r.write.id == request_id) {
            req.status = ApprovalStatus::Approved;
            req.resolved_at = Some(chrono::Utc::now());
            req.approver = Some(approver.to_string());
            req.reason = Some(reason.to_string());
            return Some(req.clone());
        }
        None
    }

    /// List pending reviews.
    pub async fn pending_reviews(&self) -> Vec<ApprovalRequest> {
        self.pending.read().await.clone()
    }
}
