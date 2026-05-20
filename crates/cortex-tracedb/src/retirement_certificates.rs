use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A cryptographic certificate proving safe, complete decommissioning.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RetirementCertificate {
    pub certificate_id: Uuid,
    pub source_system_id: Option<Uuid>,

    pub fields_absorbed: i32,
    pub workflows_migrated: i32,
    pub data_integrity_hash: String,        // Merkle root of all absorbed data
    pub compliance_frameworks: Option<Vec<String>>,

    pub issued_at: DateTime<Utc>,
    pub signed_by: Option<Uuid>,
    pub signature: Option<Vec<u8>>,
    pub scitt_receipt: Option<String>,
}

pub struct RetirementCertificateRepo {
    pool: PgPool,
}

impl RetirementCertificateRepo {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn issue(
        &self,
        cert: &RetirementCertificate,
    ) -> Result<RetirementCertificate, sqlx::Error> {
        sqlx::query_as::<_, RetirementCertificate>(
            r#"INSERT INTO retirement_certificates (
                   source_system_id, fields_absorbed, workflows_migrated,
                   data_integrity_hash, compliance_frameworks,
                   signed_by, signature, scitt_receipt
               ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) RETURNING *"#
        )
        .bind(cert.source_system_id)
        .bind(cert.fields_absorbed).bind(cert.workflows_migrated)
        .bind(&cert.data_integrity_hash).bind(&cert.compliance_frameworks)
        .bind(cert.signed_by).bind(&cert.signature).bind(&cert.scitt_receipt)
        .fetch_one(&self.pool)
        .await
    }
}
