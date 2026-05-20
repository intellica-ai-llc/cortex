use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// An edge connecting two decision traces, with write‑time program behaviour.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TraceEdge {
    pub edge_id: Uuid,
    pub from_trace_id: Uuid,
    pub to_trace_id: Uuid,
    pub edge_type: String,              // 'caused_by','informs','contradicts','supersedes'
    pub on_insert_behavior: Option<String>,
    pub content_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct TraceEdgeRepo {
    pool: PgPool,
}

impl TraceEdgeRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn insert(&self, edge: &TraceEdge) -> Result<TraceEdge, sqlx::Error> {
        sqlx::query_as::<_, TraceEdge>(
            r#"INSERT INTO trace_edges (
                   from_trace_id, to_trace_id, edge_type, on_insert_behavior, content_hash
               ) VALUES ($1,$2,$3,$4,$5)
               ON CONFLICT (from_trace_id, to_trace_id, edge_type) DO NOTHING
               RETURNING *"#
        )
        .bind(edge.from_trace_id).bind(edge.to_trace_id)
        .bind(&edge.edge_type).bind(&edge.on_insert_behavior)
        .bind(&edge.content_hash)
        .fetch_one(&self.pool)
        .await
    }
}
