use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A single decision trace (AER + DES compliant).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DecisionTrace {
    pub trace_id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub agent_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,

    // AER Core Fields
    pub intent: String,
    pub observation: serde_json::Value,
    pub inference: Option<serde_json::Value>,
    pub evidence_chain: Option<serde_json::Value>,

    // DES Compliance
    pub decision_type: String,
    pub actor_type: String,            // 'human', 'agent', 'hybrid'
    pub governance_tier: String,       // 'lightweight', 'sampled', 'full'
    pub policy_version: Option<String>,

    // Behavioral Abstraction
    pub behavioral_token: String,
    pub source_application: String,
    pub source_schema_ref: Option<Uuid>,
    pub source_value_before: Option<serde_json::Value>,
    pub source_value_after: Option<serde_json::Value>,

    // AER Versioned Plans
    pub plan_version: i32,
    pub revision_rationale: Option<String>,
    pub confidence_score: Option<f64>,
    pub delegation_chain: Option<serde_json::Value>,
    pub verdict: Option<serde_json::Value>,

    // Context Graph Linkage
    pub parent_trace_ids: Option<Vec<Uuid>>,
    pub child_trace_ids: Option<Vec<Uuid>>,
    pub content_hash: Option<String>,

    pub created_at: DateTime<Utc>,
}

/// Repository for persisting decision traces.
pub struct DecisionTraceRepo {
    pool: PgPool,
}

impl DecisionTraceRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    /// Insert a new decision trace and return the full row.
    pub async fn insert(&self, trace: &DecisionTrace) -> Result<DecisionTrace, sqlx::Error> {
        sqlx::query_as::<_, DecisionTrace>(
            r#"INSERT INTO decision_traces (
                   user_id, session_id, agent_id, intent, observation, inference, evidence_chain,
                   decision_type, actor_type, governance_tier, policy_version,
                   behavioral_token, source_application, source_schema_ref,
                   source_value_before, source_value_after,
                   plan_version, revision_rationale, confidence_score, delegation_chain, verdict,
                   parent_trace_ids, child_trace_ids, content_hash
               ) VALUES (
                   $1, $2, $3, $4, $5, $6, $7,
                   $8, $9, $10, $11,
                   $12, $13, $14,
                   $15, $16,
                   $17, $18, $19, $20, $21,
                   $22, $23, $24
               )
               RETURNING *"#
        )
        .bind(trace.user_id)
        .bind(trace.session_id)
        .bind(trace.agent_id)
        .bind(&trace.intent)
        .bind(&trace.observation)
        .bind(&trace.inference)
        .bind(&trace.evidence_chain)
        .bind(&trace.decision_type)
        .bind(&trace.actor_type)
        .bind(&trace.governance_tier)
        .bind(&trace.policy_version)
        .bind(&trace.behavioral_token)
        .bind(&trace.source_application)
        .bind(trace.source_schema_ref)
        .bind(&trace.source_value_before)
        .bind(&trace.source_value_after)
        .bind(trace.plan_version)
        .bind(&trace.revision_rationale)
        .bind(trace.confidence_score)
        .bind(&trace.delegation_chain)
        .bind(&trace.verdict)
        .bind(&trace.parent_trace_ids)
        .bind(&trace.child_trace_ids)
        .bind(&trace.content_hash)
        .fetch_one(&self.pool)
        .await
    }

    /// Query traces for a user within a time range.
    pub async fn by_user_time(
        &self,
        user_id: Uuid,
        since: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<DecisionTrace>, sqlx::Error> {
        sqlx::query_as::<_, DecisionTrace>(
            "SELECT * FROM decision_traces WHERE user_id = $1 AND timestamp >= $2 ORDER BY timestamp DESC LIMIT $3"
        )
        .bind(user_id)
        .bind(since)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }
}
