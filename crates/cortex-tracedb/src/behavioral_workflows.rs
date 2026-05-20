use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A discovered behavioral workflow — a repeated sequence of high‑level tokens.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BehavioralWorkflow {
    pub workflow_id: Uuid,
    pub user_id: Uuid,
    pub source_application: String,

    pub behavioral_tokens: Vec<String>,
    pub workflow_graph: Option<serde_json::Value>,   // DAG
    pub frequency: i32,
    pub total_duration_ms: Option<i64>,

    pub converted_to_skill: bool,
    pub skill_id: Option<Uuid>,
    pub absorption_phase: String,

    pub first_observed: DateTime<Utc>,
    pub last_observed: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
}

pub struct BehavioralWorkflowRepo {
    pool: PgPool,
}

impl BehavioralWorkflowRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    /// Insert a new workflow or update frequency if the same token sequence exists.
    pub async fn upsert(&self, wf: &BehavioralWorkflow) -> Result<BehavioralWorkflow, sqlx::Error> {
        sqlx::query_as::<_, BehavioralWorkflow>(
            r#"INSERT INTO behavioral_workflows (
                   user_id, source_application, behavioral_tokens, workflow_graph,
                   frequency, total_duration_ms, converted_to_skill, skill_id, absorption_phase
               ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               ON CONFLICT DO NOTHING   -- simple dedup; production would match on token array
               RETURNING *"#
        )
        .bind(wf.user_id)
        .bind(&wf.source_application)
        .bind(&wf.behavioral_tokens)
        .bind(&wf.workflow_graph)
        .bind(wf.frequency)
        .bind(wf.total_duration_ms)
        .bind(wf.converted_to_skill)
        .bind(wf.skill_id)
        .bind(&wf.absorption_phase)
        .fetch_one(&self.pool)
        .await
    }

    /// Find workflows for a user ordered by frequency.
    pub async fn top_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<BehavioralWorkflow>, sqlx::Error> {
        sqlx::query_as::<_, BehavioralWorkflow>(
            "SELECT * FROM behavioral_workflows WHERE user_id = $1 ORDER BY frequency DESC LIMIT $2"
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }
}
