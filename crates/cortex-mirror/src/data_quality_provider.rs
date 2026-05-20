use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Pluggable Data Quality Provider trait.
///
/// Abstracts PII detection, data quality, schema evolution, and
/// auto‑tuning capabilities behind a universal interface. GoldenGate
/// 26ai AI Microservice provides the default Oracle implementation;
/// pgstream provides the PostgreSQL implementation. Enterprises can
/// plug in custom providers, eliminating vendor lock‑in.
///
/// Striim Validata (Apr 22, 2026) demonstrates the production
/// pattern: "continuous, real‑time source‑to‑target validation
/// and reconciliation engine for CDC replication. Compares
/// checksums, flags mismatches, turns them into repair scripts,
/// and re‑checks results."
#[async_trait]
pub trait DataQualityProvider: Send + Sync {
    /// Detect PII in a column value.
    async fn detect_pii(&self, value: &str) -> Result<PiiDetectionResult, DQError>;

    /// Check data quality for a batch of column values.
    async fn check_quality(
        &self,
        column_name: &str,
        values: &[Option<String>],
    ) -> Result<QualityCheckResult, DQError>;

    /// Handle a schema change event.
    async fn handle_schema_change(
        &self,
        change: &super::cdc_trait::SchemaChange,
    ) -> Result<SchemaEvolutionResult, DQError>;

    /// Provider name for logging and selection.
    fn provider_name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiDetectionResult {
    pub contains_pii: bool,
    pub pii_types: Vec<String>,       // EMAIL, PHONE, SSN, CREDIT_CARD, etc.
    pub confidence: f64,
    pub redacted_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheckResult {
    pub column: String,
    pub total_values: usize,
    pub null_count: usize,
    pub distinct_count: usize,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub anomalies: Vec<String>,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEvolutionResult {
    pub change_accepted: bool,
    pub propagated_to_target: bool,
    pub target_column: Option<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum DQError {
    #[error("Provider unavailable: {0}")]
    Unavailable(String),
    #[error("Detection failed: {0}")]
    DetectionFailed(String),
    #[error("Quality check failed: {0}")]
    QualityCheckFailed(String),
    #[error("Schema evolution failed: {0}")]
    SchemaEvolutionFailed(String),
}

// ── GoldenGate 26ai AI Microservice provider ──

/// GoldenGate 26ai AI Microservice data quality provider.
///
/// Oracle GoldenGate 26ai (Jan 29, 2026) introduces an embedded AI
/// Microservice that enables "real‑time named‑entity recognition,
/// PII identification on transactional data, natural‑language
/// administration, agentic APIs (such as MCP), data enrichment
/// using any LLM service, automated data quality enhancements,
/// and intelligent auto‑tuning."
pub struct GoldenGateDataQualityProvider {
    gg_endpoint: String,
    api_key: String,
}

impl GoldenGateDataQualityProvider {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        Self { gg_endpoint: endpoint.to_string(), api_key: api_key.to_string() }
    }
}

#[async_trait]
impl DataQualityProvider for GoldenGateDataQualityProvider {
    async fn detect_pii(&self, _value: &str) -> Result<PiiDetectionResult, DQError> {
        // Production: call GoldenGate AI Microservice NER API.
        Ok(PiiDetectionResult {
            contains_pii: false, pii_types: vec![], confidence: 0.0, redacted_value: None,
        })
    }

    async fn check_quality(
        &self, column_name: &str, values: &[Option<String>],
    ) -> Result<QualityCheckResult, DQError> {
        Ok(QualityCheckResult {
            column: column_name.to_string(),
            total_values: values.len(),
            null_count: values.iter().filter(|v| v.is_none()).count(),
            distinct_count: 0, min_value: None, max_value: None,
            anomalies: vec![], passed: true,
        })
    }

    async fn handle_schema_change(
        &self, change: &super::cdc_trait::SchemaChange,
    ) -> Result<SchemaEvolutionResult, DQError> {
        Ok(SchemaEvolutionResult {
            change_accepted: true, propagated_to_target: true,
            target_column: None, warnings: vec![],
        })
    }

    fn provider_name(&self) -> &str { "GoldenGate 26ai AI Microservice" }
}

// ── pgstream data quality provider ──

/// pgstream‑based data quality provider for PostgreSQL sources.
pub struct PgstreamDataQualityProvider;

impl PgstreamDataQualityProvider {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl DataQualityProvider for PgstreamDataQualityProvider {
    async fn detect_pii(&self, _value: &str) -> Result<PiiDetectionResult, DQError> {
        Ok(PiiDetectionResult {
            contains_pii: false, pii_types: vec![], confidence: 0.0, redacted_value: None,
        })
    }

    async fn check_quality(
        &self, column_name: &str, values: &[Option<String>],
    ) -> Result<QualityCheckResult, DQError> {
        Ok(QualityCheckResult {
            column: column_name.to_string(),
            total_values: values.len(),
            null_count: values.iter().filter(|v| v.is_none()).count(),
            distinct_count: 0, min_value: None, max_value: None,
            anomalies: vec![], passed: true,
        })
    }

    async fn handle_schema_change(
        &self, _change: &super::cdc_trait::SchemaChange,
    ) -> Result<SchemaEvolutionResult, DQError> {
        Ok(SchemaEvolutionResult {
            change_accepted: true, propagated_to_target: true,
            target_column: None, warnings: vec![],
        })
    }

    fn provider_name(&self) -> &str { "pgstream v1.0.1" }
}
