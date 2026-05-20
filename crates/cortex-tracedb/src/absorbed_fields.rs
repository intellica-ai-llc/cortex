use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A field discovered by the Observational Agent and tracked through six phases.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AbsorbedField {
    pub field_id: Uuid,
    pub source_application: String,
    pub source_database: String,
    pub source_schema: String,
    pub source_table: String,
    pub source_column: String,

    // Schema Grounding
    pub semantic_label: Option<String>,
    pub field_description: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub ontology_category: Option<String>,

    // Type & Constraint Discovery
    pub field_type: String,
    pub field_length: Option<i32>,
    pub is_nullable: bool,
    pub validation_rules: Option<serde_json::Value>,

    // Observation Statistics
    pub first_observed_at: Option<DateTime<Utc>>,
    pub last_observed_at: Option<DateTime<Utc>>,
    pub observation_count: i32,
    pub unique_users: i32,

    // Six‑Phase Absorption Status
    pub absorption_status: String,   // 'observing' | 'mirroring' | 'absorbed' | 'genesis' | 'replaced' | 'retired'

    // CDC Integration
    pub cdc_connector_id: Option<String>,
    pub cdc_sync_started: Option<DateTime<Utc>>,
    pub cdc_last_sync: Option<DateTime<Utc>>,
    pub cdc_sync_latency_ms: Option<i32>,

    // Auto‑Evolution (ThemisDB pattern)
    pub cortex_table: Option<String>,
    pub cortex_column: Option<String>,
    pub schema_version: i32,
    pub last_schema_change: Option<DateTime<Utc>>,
    pub evolution_history: Option<serde_json::Value>,

    // Governance
    pub contains_pii: bool,
    pub pii_type: Option<String>,
    pub retention_policy: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct AbsorbedFieldRepo {
    pool: PgPool,
}

impl AbsorbedFieldRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    /// Insert or update an absorbed field (upsert on natural key).
    pub async fn upsert(&self, field: &AbsorbedField) -> Result<AbsorbedField, sqlx::Error> {
        sqlx::query_as::<_, AbsorbedField>(
            r#"INSERT INTO absorbed_fields (
                   source_application, source_database, source_schema, source_table, source_column,
                   semantic_label, field_description, embedding, ontology_category,
                   field_type, field_length, is_nullable, validation_rules,
                   first_observed_at, last_observed_at, observation_count, unique_users,
                   absorption_status,
                   cdc_connector_id, cdc_sync_started, cdc_last_sync, cdc_sync_latency_ms,
                   cortex_table, cortex_column, schema_version, last_schema_change, evolution_history,
                   contains_pii, pii_type, retention_policy
               ) VALUES (
                   $1, $2, $3, $4, $5,
                   $6, $7, $8, $9,
                   $10, $11, $12, $13,
                   $14, $15, $16, $17,
                   $18,
                   $19, $20, $21, $22,
                   $23, $24, $25, $26, $27,
                   $28, $29, $30
               )
               ON CONFLICT (source_application, source_database, source_schema, source_table, source_column)
               DO UPDATE SET
                   semantic_label = EXCLUDED.semantic_label,
                   field_description = EXCLUDED.field_description,
                   embedding = EXCLUDED.embedding,
                   observation_count = absorbed_fields.observation_count + 1,
                   last_observed_at = EXCLUDED.last_observed_at,
                   absorption_status = EXCLUDED.absorption_status,
                   updated_at = now()
               RETURNING *"#
        )
        .bind(&field.source_application)
        .bind(&field.source_database)
        .bind(&field.source_schema)
        .bind(&field.source_table)
        .bind(&field.source_column)
        .bind(&field.semantic_label)
        .bind(&field.field_description)
        .bind(&field.embedding)
        .bind(&field.ontology_category)
        .bind(&field.field_type)
        .bind(field.field_length)
        .bind(field.is_nullable)
        .bind(&field.validation_rules)
        .bind(field.first_observed_at)
        .bind(field.last_observed_at)
        .bind(field.observation_count)
        .bind(field.unique_users)
        .bind(&field.absorption_status)
        .bind(&field.cdc_connector_id)
        .bind(field.cdc_sync_started)
        .bind(field.cdc_last_sync)
        .bind(field.cdc_sync_latency_ms)
        .bind(&field.cortex_table)
        .bind(&field.cortex_column)
        .bind(field.schema_version)
        .bind(field.last_schema_change)
        .bind(&field.evolution_history)
        .bind(field.contains_pii)
        .bind(&field.pii_type)
        .bind(&field.retention_policy)
        .fetch_one(&self.pool)
        .await
    }

    /// List fields by absorption status.
    pub async fn by_status(&self, status: &str) -> Result<Vec<AbsorbedField>, sqlx::Error> {
        sqlx::query_as::<_, AbsorbedField>(
            "SELECT * FROM absorbed_fields WHERE absorption_status = $1"
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
    }
}
