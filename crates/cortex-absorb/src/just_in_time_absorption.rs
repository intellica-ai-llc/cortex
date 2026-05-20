use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::Utc;

/// Just‑In‑Time Field Absorption — absorbs fields based on
/// observation frequency, user role, and business criticality.
///
/// When a field's observation_count exceeds a configurable
/// threshold, or when the Data Gravity Scorer (v9 TraceDB)
/// elevates it due to high fan‑out, the field transitions from
/// 'mirroring' to 'absorbed' and a dedicated absorption table
/// column is created via the FastProvisioner.
///
/// The absorption is not a snapshot — it is a continuous CDC
/// stream using GoldenGate 26ai (Oracle) or pgstream (PostgreSQL)
/// Automatic Schema Evolution, or the Cortex Vault backup‑based
/// pathway for vendor‑independent extraction.
pub struct JustInTimeAbsorption {
    pool: PgPool,
    /// Minimum observation_count before a field is eligible.
    min_observations: i32,
    /// Minimum unique users before a field is eligible.
    min_unique_users: i32,
    /// Whether to use the Data Gravity Scorer for prioritisation.
    use_gravity_scorer: bool,
}

/// The decision for a single field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsorptionDecision {
    pub field_id: uuid::Uuid,
    pub source_table: String,
    pub source_column: String,
    pub absorb: bool,
    pub reason: AbsorptionReason,
    pub priority: u8, // 1 = highest, 100 = lowest
    pub decided_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbsorptionReason {
    /// Field has been observed frequently enough.
    FrequencyMet { observations: i32, threshold: i32 },
    /// Field is referenced by many other tables (high fan‑out).
    HighGravity { fan_out: usize },
    /// Field is critical for regulatory compliance.
    RegulatoryRequired { regulation: String },
    /// Field was manually flagged for absorption.
    ManualOverride { by: String },
    /// Not yet eligible.
    InsufficientData,
}

impl JustInTimeAbsorption {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            min_observations: 10,
            min_unique_users: 3,
            use_gravity_scorer: true,
        }
    }

    /// Evaluate whether a field should be absorbed.
    ///
    /// Criteria:
    ///   1. observation_count >= min_observations
    ///   2. unique_users >= min_unique_users
    ///   3. OR contains_pii == true (regulatory risk)
    ///   4. OR data gravity score is in the top 20th percentile
    pub async fn evaluate(
        &self,
        field_id: uuid::Uuid,
        observation_count: i32,
        unique_users: i32,
        contains_pii: bool,
        fan_out: usize,
    ) -> AbsorptionDecision {
        let mut reasons = Vec::new();

        if observation_count >= self.min_observations {
            reasons.push(AbsorptionReason::FrequencyMet {
                observations: observation_count,
                threshold: self.min_observations,
            });
        }

        if contains_pii {
            reasons.push(AbsorptionReason::RegulatoryRequired {
                regulation: "GDPR/EU AI Act".into(),
            });
        }

        if fan_out > 5 {
            reasons.push(AbsorptionReason::HighGravity { fan_out });
        }

        let absorb = !reasons.is_empty() || unique_users >= self.min_unique_users;
        let reason = if reasons.is_empty() {
            AbsorptionReason::InsufficientData
        } else {
            reasons.into_iter().next().unwrap()
        };

        let priority = if contains_pii || fan_out > 10 {
            1
        } else if observation_count > 50 {
            10
        } else {
            50
        };

        AbsorptionDecision {
            field_id,
            source_table: String::new(),
            source_column: String::new(),
            absorb,
            reason,
            priority,
            decided_at: Utc::now(),
        }
    }

    /// Transition a field from 'mirroring' to 'absorbed' in TraceDB.
    pub async fn absorb_field(&self, field_id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE absorbed_fields
               SET absorption_status = 'absorbed',
                   absorbed_at = NOW(),
                   updated_at = NOW()
               WHERE field_id = $1
                 AND absorption_status = 'mirroring'"#
        )
        .bind(field_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
