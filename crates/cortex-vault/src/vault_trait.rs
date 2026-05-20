use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Universal backup extraction trait.
#[async_trait]
pub trait VaultBackend: Send + Sync {
    /// Discover all tables, columns, and types within a backup file.
    async fn discover_schema(&self, backup_path: &str) -> Result<SchemaDiscovery, VaultError>;

    /// Extract data for a set of selected tables into a stream.
    async fn extract_data(
        &self,
        tables: &[TableSelection],
        progress: &dyn ProgressCallback,
    ) -> Result<ExtractionResult, VaultError>;

    /// Validate extracted data against a checksum manifest.
    async fn validate_extraction(
        &self,
        result: &ExtractionResult,
        manifest: &ChecksumManifest,
    ) -> Result<ValidationReport, VaultError>;

    /// List supported backup format versions.
    fn supported_versions(&self) -> Vec<BackupVersion>;

    /// Estimate extraction time based on backup size.
    fn estimate_duration(&self, size_bytes: u64) -> std::time::Duration;
}

/// Schema discovered from a backup file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDiscovery {
    pub source: String,
    pub tables: Vec<TableSchema>,
    pub total_rows_estimate: u64,
    pub extraction_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Table schema within a backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
    pub primary_keys: Vec<String>,
    pub foreign_keys: Vec<ForeignKeyRef>,
}

/// Column schema within a table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub default_value: Option<String>,
}

/// Foreign key reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyRef {
    pub column: String,
    pub ref_table: String,
    pub ref_column: String,
}

/// Extraction request for specific tables.
#[derive(Debug, Clone)]
pub struct TableSelection {
    pub table_name: String,
    pub columns: Option<Vec<String>>, // None = all columns
    pub row_filter: Option<String>,   // SQL-compatible filter
}

/// Callback for progress reporting.
pub trait ProgressCallback: Send + Sync {
    fn report(&self, stage: &str, pct: f64, rows_extracted: u64);
}

/// Result of a full extraction.
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    pub source: String,
    pub tables: Vec<TableExtract>,
    pub total_rows: u64,
    pub duration_ms: u64,
}

/// Extracted data for a single table.
#[derive(Debug, Clone)]
pub struct TableExtract {
    pub table_name: String,
    pub columns: Vec<ColumnSchema>,
    pub row_data: Vec<Vec<Option<String>>>,
    pub row_count: u64,
}

/// Checksum manifest for validation.
#[derive(Debug, Clone)]
pub struct ChecksumManifest {
    pub source: String,
    pub tables: Vec<TableChecksum>,
}

#[derive(Debug, Clone)]
pub struct TableChecksum {
    pub table_name: String,
    pub row_count: u64,
    pub checksum: String, // BLAKE3 over canonical row representation
}

/// Validation report.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub passed: bool,
    pub total_tables_checked: usize,
    pub mismatches: Vec<Mismatch>,
}

#[derive(Debug, Clone)]
pub struct Mismatch {
    pub table: String,
    pub expected_checksum: String,
    pub actual_checksum: String,
}

/// Backup version descriptor.
#[derive(Debug, Clone)]
pub struct BackupVersion {
    pub vendor: String,
    pub version: String,
    pub format: String,
}

/// Unified error type.
#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("Backup file not found: {0}")]
    FileNotFound(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Extraction error: {0}")]
    ExtractionError(String),
    #[error("Checksum mismatch: {0}")]
    ChecksumMismatch(String),
}
