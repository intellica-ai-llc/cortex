use async_trait::async_trait;
use super::cdc_trait::*;

/// pgstream v1.0.1 adapter – stateless DDL replication for PostgreSQL.
///
/// Captures DDL via event triggers and emits them as logical WAL
/// messages via pg_logical_emit_message. No schema log table.
/// DDL itself is the source of truth.
pub struct PgstreamAdapter {
    binary_path: String,
}

impl PgstreamAdapter {
    pub fn new(binary_path: &str) -> Self {
        Self { binary_path: binary_path.to_string() }
    }
}

#[async_trait]
impl CdcBackend for PgstreamAdapter {
    async fn initialize(&self, config: &MirrorConfig) -> Result<CdcHandle, CdcError> {
        // Production: spawn pgstream process with --source and --target args.
        Ok(CdcHandle { source: config.source_name.clone(), snapshot_lsn: None })
    }

    async fn bulk_load(&self, handle: &CdcHandle) -> Result<BulkLoadResult, CdcError> {
        Ok(BulkLoadResult { rows_loaded: 0, duration_ms: 0, snapshot_lsn: "0".into() })
    }

    async fn start_streaming(&self, handle: &CdcHandle) -> Result<StreamingHandle, CdcError> {
        Ok(StreamingHandle { source: handle.source.clone(), current_lsn: None, started_at: chrono::Utc::now() })
    }

    async fn pause(&self, _h: &StreamingHandle) -> Result<(), CdcError> { Ok(()) }
    async fn resume(&self, _h: &StreamingHandle) -> Result<(), CdcError> { Ok(()) }
    async fn get_latency(&self, _h: &StreamingHandle) -> Result<u64, CdcError> { Ok(0) }

    async fn handle_schema_change(&self, _h: &StreamingHandle, change: SchemaChange) -> Result<(), CdcError> {
        // pgstream emits DDL changes as WAL messages; here we just acknowledge.
        tracing::info!(change = ?change, "pgstream schema change received");
        Ok(())
    }

    async fn teardown(&self, _handle: CdcHandle) -> Result<(), CdcError> { Ok(()) }
    fn source_type(&self) -> SourceDbType { SourceDbType::PostgreSQL }
}
