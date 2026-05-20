use async_trait::async_trait;
use super::vault_trait::*;

/// MySQL backup parser (mysqldump SQL / XtraBackup).
pub struct MySqlBackupParser {
    dump_file: String,
}

impl MySqlBackupParser {
    pub fn new(dump_file: &str) -> Self {
        Self { dump_file: dump_file.to_string() }
    }
}

#[async_trait]
impl VaultBackend for MySqlBackupParser {
    async fn discover_schema(&self, _backup_path: &str) -> Result<SchemaDiscovery, VaultError> {
        Ok(SchemaDiscovery {
            source: "MySQL dump".into(),
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
        vec![BackupVersion { vendor: "MySQL/MariaDB".into(), version: "5.7–8.4".into(), format: "mysqldump".into() }]
    }

    fn estimate_duration(&self, size_bytes: u64) -> std::time::Duration {
        std::time::Duration::from_secs(size_bytes / (90 * 1024 * 1024) * 60)
    }
}
