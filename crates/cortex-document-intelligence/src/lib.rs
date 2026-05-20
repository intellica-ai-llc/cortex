//! Cortex Document Intelligence – scan → extract → compliance feedback.
//!
//! Pipeline: field worker scans report → Kreuzberg extracts text/tables
//! → docsingest screens for PII/PHI/CUI → compliance‑checker‑algo cross‑
//! references against Knowledge Snap industry benchmarks → NL feedback
//! returned to dashboard → document archived in TraceDB with hash‑chain
//! integrity and SCITT anchoring.

pub mod doc_ingestor;
pub mod compliance_screener;
pub mod benchmark_cross_reference;
pub mod feedback_generator;
pub mod document_lineage;
pub mod scan_to_dashboard;
