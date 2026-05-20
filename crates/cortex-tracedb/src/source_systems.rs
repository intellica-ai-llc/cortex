use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};

/// A legacy source system tracked through the six‑phase absorption lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SourceSystem {
    pub system_id: Uuid,
    pub system_name: String,
    pub system_type: String,            // 'EAM', 'HR', 'ERP', 'CRM', 'SCADA'
    pub vendor: Option<String>,

    pub db_connection_string: Option<String>,
    pub mcp_connector_name: Option<String>,

    pub total_tables: Option<i32>,
    pub total_columns: Option<i32>,
    pub fields_discovered: i32,
    pub fields_absorbed: i32,

    /// Auto‑computed: (fields_absorbed / total_columns) * 100
    pub absorption_pct: Option<f64>,
    pub absorption_phase: String,       // 'observing' … 'retired'

    pub projected_retirement_date: Option<NaiveDate>,
    pub actual_retirement_date: Option<NaiveDate>,
    pub license_cost_annual: Option<rust_decimal::Decimal>,
    pub license_savings_ytd: Option<rust_decimal::Decimal>,

    pub created_at: DateTime<Utc>,
}

pub struct SourceSystemRepo {
    pool: PgPool,
}

impl SourceSystemRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn register(&self, ss: &SourceSystem) -> Result<SourceSystem, sqlx::Error> {
        sqlx::query_as::<_, SourceSystem>(
            r#"INSERT INTO source_systems (
                   system_name, system_type, vendor,
                   db_connection_string, mcp_connector_name,
                   total_tables, total_columns, fields_discovered, fields_absorbed,
                   absorption_phase, projected_retirement_date,
                   license_cost_annual, license_savings_ytd
               ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
               RETURNING *"#
        )
        .bind(&ss.system_name).bind(&ss.system_type).bind(&ss.vendor)
        .bind(&ss.db_connection_string).bind(&ss.mcp_connector_name)
        .bind(ss.total_tables).bind(ss.total_columns)
        .bind(ss.fields_discovered).bind(ss.fields_absorbed)
        .bind(&ss.absorption_phase).bind(ss.projected_retirement_date)
        .bind(ss.license_cost_annual).bind(ss.license_savings_ytd)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn by_phase(&self, phase: &str) -> Result<Vec<SourceSystem>, sqlx::Error> {
        sqlx::query_as::<_, SourceSystem>(
            "SELECT * FROM source_systems WHERE absorption_phase = $1"
        ).bind(phase).fetch_all(&self.pool).await
    }
}
