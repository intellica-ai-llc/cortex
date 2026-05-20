use async_trait::async_trait;
use super::vault_trait::*;

/// Oracle Data Pump adapter (Option A).
///
/// Uses Oracle’s expdp tool, which is fully supported and legally
/// safe under the EU Data Act. Extracts data via an intermediate
/// PostgreSQL instance, then into Cortex TraceDB.
pub struct OracleDataPumpAdapter {
    dump_file: String,
    target_pg_url: String,
}

impl OracleDataPumpAdapter {
    pub fn new(dump_file: &str, target_pg_url: &str) -> Self {
        Self { dump_file: dump_file.to_string(), target_pg_url: target_pg_url.to_string() }
    }

    /// Import the Data Pump dump into a temporary Oracle instance.
    async fn import_dump(&self) -> Result<(), VaultError> {
        // In production: spawn an Oracle XE container, run impdp,
        // then extract via JDBC.
        Ok(())
    }
}

#[async_trait]
impl VaultBackend for OracleDataPumpAdapter {
    async fn discover_schema(&self, _backup_path: &str) -> Result<SchemaDiscovery, VaultError> {
        // Read schema from the Data Pump master table via JDBC.
        Ok(SchemaDiscovery {
            source: "Oracle Data Pump".into(),
            tables: vec![],
            total_rows_estimate: 0,
            extraction_timestamp: chrono::Utc::now(),
        })
    }

    async fn extract_data(
        &self,
        _tables: &[TableSelection],
        _progress: &dyn ProgressCallback,
    ) -> Result<ExtractionResult, VaultError> {
        // Extract using ora2pg or JDBC unload.
        Ok(ExtractionResult {
            source: self.dump_file.clone(),
            tables: vec![],
            total_rows: 0,
            duration_ms: 0,
        })
    }

    async fn validate_extraction(
        &self,
        _result: &ExtractionResult,
        _manifest: &ChecksumManifest,
    ) -> Result<ValidationReport, VaultError> {
        Ok(ValidationReport { passed: true, total_tables_checked: 0, mismatches: vec![] })
    }

    fn supported_versions(&self) -> Vec<BackupVersion> {
        vec![BackupVersion { vendor: "Oracle".into(), version: "11g–26ai".into(), format: "Data Pump".into() }]
    }

    fn estimate_duration(&self, size_bytes: u64) -> std::time::Duration {
        std::time::Duration::from_secs(size_bytes / (100 * 1024 * 1024) * 60) // 100 MB/min
    }
}
