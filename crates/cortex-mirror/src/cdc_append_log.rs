use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};

/// Pinterest‑style two‑tier CDC storage: immutable append log.
///
/// Pinterest’s CDC‑powered ingestion framework (InfoQ, Feb 26, 2026)
/// separates CDC tables from base tables: "CDC tables act as
/// append‑only ledgers with sub‑5‑minute latency, while base tables
/// maintain full historical snapshots updated via Spark Merge Into
/// operations every 15–60 minutes." This design reduces data volume
/// by 95% — only changed records are processed, not full‑table
/// snapshots.
///
/// Pinterest standardized on Iceberg’s Merge on Read strategy over
/// Copy on Write to control storage costs at petabyte scale:
/// "Copy on Write introduced significantly higher storage costs."
/// Cortex adopts the same strategy for its absorption tables.
///
/// The CDC append log is the source of truth. The base snapshot
/// is a periodically refreshed materialisation. Agents query the
/// base snapshot for current state; the CDC append log is used
/// for audit trails and temporal queries.
pub struct CdcAppendLog {
    pool: PgPool,
}

/// A single immutable CDC event stored in the append log.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CdcLogEntry {
    pub id: uuid::Uuid,
    pub source: String,
    pub table_name: String,
    pub operation: String,          // INSERT, UPDATE, DELETE
    pub primary_key: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub transaction_id: String,
    pub lsn: Option<String>,
    pub ingested_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdcLogStats {
    pub total_entries: i64,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
    pub entries_by_table: std::collections::HashMap<String, i64>,
}

impl CdcAppendLog {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    /// Append a CDC event to the immutable log.
    pub async fn append(&self, entry: &CdcLogEntry) -> Result<CdcLogEntry, sqlx::Error> {
        sqlx::query_as::<_, CdcLogEntry>(
            r#"INSERT INTO cdc_append_log (
                   source, table_name, operation, primary_key,
                   old_values, new_values, transaction_id, lsn
               ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               RETURNING *"#
        )
        .bind(&entry.source).bind(&entry.table_name)
        .bind(&entry.operation).bind(&entry.primary_key)
        .bind(&entry.old_values).bind(&entry.new_values)
        .bind(&entry.transaction_id).bind(&entry.lsn)
        .fetch_one(&self.pool)
        .await
    }

    /// Query the log for a time range (used by temporal queries and audits).
    pub async fn query_range(
        &self,
        table: &str,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<Vec<CdcLogEntry>, sqlx::Error> {
        sqlx::query_as::<_, CdcLogEntry>(
            r#"SELECT * FROM cdc_append_log
               WHERE table_name = $1 AND ingested_at >= $2 AND ingested_at <= $3
               ORDER BY ingested_at ASC"#
        )
        .bind(table).bind(since).bind(until)
        .fetch_all(&self.pool)
        .await
    }

    /// Compute statistics about the log.
    pub async fn stats(&self) -> Result<CdcLogStats, sqlx::Error> {
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM cdc_append_log"
        ).fetch_one(&self.pool).await?;

        let oldest: Option<(DateTime<Utc>,)> = sqlx::query_as(
            "SELECT MIN(ingested_at) FROM cdc_append_log"
        ).fetch_one(&self.pool).await?;

        let newest: Option<(DateTime<Utc>,)> = sqlx::query_as(
            "SELECT MAX(ingested_at) FROM cdc_append_log"
        ).fetch_one(&self.pool).await?;

        Ok(CdcLogStats {
            total_entries: total.0,
            oldest_entry: oldest.map(|o| o.0),
            newest_entry: newest.map(|n| n.0),
            entries_by_table: std::collections::HashMap::new(),
        })
    }
}
