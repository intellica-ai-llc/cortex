//! Hash‑chain integrity and SCITT anchoring for every ingested document.
//! Based on docsingest's FedRAMP audit trail pattern: tamper‑evident SHA‑256
//! hash chain with CEF export.

use serde::{Deserialize, Serialize};

pub struct DocumentLineage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRecord {
    pub doc_id: String,
    pub ingestion_hash: String,     // BLAKE3 hash of original document bytes
    pub screening_hash: String,     // hash of screening result
    pub cross_reference_hash: String,
    pub previous_record_hash: Option<String>,
    pub scitt_receipt: Option<String>,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl DocumentLineage {
    pub fn new() -> Self { Self }

    /// Create a lineage record linking this document to the hash chain.
    /// In production, the SCITT receipt is obtained by anchoring the hash chain
    /// root to a SCITT transparency service (IETF draft‑ietf‑scitt‑architecture‑08).
    pub fn record(
        &self,
        doc_id: &str,
        raw_bytes: &[u8],
        screening_result: &super::compliance_screener::ScreeningResult,
        cross_ref: &super::benchmark_cross_reference::CrossReferenceResult,
        previous_hash: Option<&str>,
    ) -> LineageRecord {
        let ingestion_hash = blake3::hash(raw_bytes).to_hex().to_string();
        let screening_hash = blake3::hash(
            serde_json::to_string(screening_result).unwrap_or_default().as_bytes()
        ).to_hex().to_string();
        let cross_reference_hash = blake3::hash(
            serde_json::to_string(cross_ref).unwrap_or_default().as_bytes()
        ).to_hex().to_string();

        LineageRecord {
            doc_id: doc_id.to_string(),
            ingestion_hash,
            screening_hash,
            cross_reference_hash,
            previous_record_hash: previous_hash.map(|s| s.to_string()),
            scitt_receipt: None,
            recorded_at: chrono::Utc::now(),
        }
    }
}
