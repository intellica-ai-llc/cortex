use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Universal CDC backend trait. Every Mirror adapter implements this.
#[async_trait]
pub trait CdcBackend: Send + Sync {
    /// Initialise the CDC pipeline for a specific set of columns.
    async fn initialize(&self, config: &MirrorConfig) -> Result<CdcHandle, CdcError>;

    /// Start the bulk load phase (full snapshot of selected columns).
    async fn bulk_load(&self, handle: &CdcHandle) -> Result<BulkLoadResult, CdcError>;

    /// Transition from bulk load to streaming CDC.
    async fn start_streaming(&self, handle: &CdcHandle) -> Result<StreamingHandle, CdcError>;

    /// Pause streaming (e.g., during backpressure or schema freeze).
    async fn pause(&self, handle: &StreamingHandle) -> Result<(), CdcError>;

    /// Resume streaming after pause.
    async fn resume(&self, handle: &StreamingHandle) -> Result<(), CdcError>;

    /// Current sync latency in milliseconds.
    async fn get_latency(&self, handle: &StreamingHandle) -> Result<u64, CdcError>;

    /// Handle a source schema change detected during streaming.
    async fn handle_schema_change(
        &self,
        handle: &StreamingHandle,
        change: SchemaChange,
    ) -> Result<(), CdcError>;

    /// Tear down the CDC pipeline.
    async fn teardown(&self, handle: CdcHandle) -> Result<(), CdcError>;

    /// Supported source database type.
    fn source_type(&self) -> SourceDbType;
}

/// Configuration for a single mirror pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorConfig {
    pub source_name: String,
    pub source_type: SourceDbType,
    pub connection_string: String,
    pub tables: Vec<TableConfig>,
    pub target_tracedb_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConfig {
    pub schema: String,
    pub table: String,
    pub columns: Vec<String>, // empty = all
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceDbType {
    Oracle,
    PostgreSQL,
    MySQL,
    SQLServer,
    DB2,
}

/// Opaque handle for an initialised CDC pipeline (bulk phase).
pub struct CdcHandle {
    pub source: String,
    pub snapshot_lsn: Option<String>,
}

/// Handle for an active streaming CDC session.
pub struct StreamingHandle {
    pub source: String,
    pub current_lsn: Option<String>,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

/// Result of a bulk load.
#[derive(Debug, Clone)]
pub struct BulkLoadResult {
    pub rows_loaded: u64,
    pub duration_ms: u64,
    pub snapshot_lsn: String,
}

/// A schema change event detected by the CDC pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaChange {
    pub table: String,
    pub change_type: SchemaChangeType,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaChangeType {
    AddColumn,
    DropColumn,
    AlterColumn,
    AddTable,
    DropTable,
}

#[derive(Debug, thiserror::Error)]
pub enum CdcError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Unsupported source: {0}")]
    UnsupportedSource(String),
    #[error("Bulk load failed: {0}")]
    BulkLoadFailed(String),
    #[error("Streaming error: {0}")]
    StreamingError(String),
    #[error("Schema change handling failed: {0}")]
    SchemaChangeError(String),
}
