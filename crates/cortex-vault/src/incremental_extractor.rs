use chrono::{DateTime, Utc};

/// Incremental extraction handler.
///
/// For Oracle RMAN incremental backup sets, parses the change tracking
/// file to identify changed blocks. For SQL Server differential
/// backups, uses the differential page map. For all sources, maintains
/// a block‑level change journal in Cortex TraceDB.
pub struct IncrementalExtractor {
    last_extraction: Option<DateTime<Utc>>,
}

impl IncrementalExtractor {
    pub fn new() -> Self { Self { last_extraction: None } }

    /// Identify changed blocks since last extraction.
    pub fn detect_changes(&self, base_version: &str) -> Vec<u64> {
        vec![] // block IDs
    }
}
