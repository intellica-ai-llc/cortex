use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// An agent‑safe branch for experimenting on absorbed data.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AbsorptionBranch {
    pub branch_id: Uuid,
    pub source_system_id: Option<Uuid>,
    pub base_branch_id: Option<Uuid>,

    pub created_by_agent_id: Option<Uuid>,
    pub branch_purpose: String,         // 'agent_experiment','qc_validation','what_if_simulation'

    pub created_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
    pub merge_status: String,           // 'active','merged','abandoned'
}

pub struct AbsorptionBranchRepo {
    pool: PgPool,
}

impl AbsorptionBranchRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn create_branch(&self, branch: &AbsorptionBranch) -> Result<AbsorptionBranch, sqlx::Error> {
        sqlx::query_as::<_, AbsorptionBranch>(
            r#"INSERT INTO absorption_branches (
                   source_system_id, base_branch_id, created_by_agent_id, branch_purpose
               ) VALUES ($1,$2,$3,$4) RETURNING *"#
        )
        .bind(branch.source_system_id).bind(branch.base_branch_id)
        .bind(branch.created_by_agent_id).bind(&branch.branch_purpose)
        .fetch_one(&self.pool)
        .await
    }
}
