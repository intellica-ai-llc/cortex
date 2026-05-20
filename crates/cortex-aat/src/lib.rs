//! Cortex AAT™ — IETF‑compliant Agent Audit Trails.
//!
//! Generates JSON records conforming to the IETF Agent Audit Trail
//! specification (May 6, 2026) and the Compliance Profile of Signed
//! Action Receipts for AI Agents. Every agent action — tool calls,
//! research queries, report generation, data access — receives a
//! cryptographically verifiable, standards‑compliant audit trail.

pub mod aat_formatter;
pub mod signed_receipt;
pub mod jurisdiction_mapper;
pub mod scitt_anchoring;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AATEngine {
    pub formatter: Arc<aat_formatter::AATFormatter>,
    pub receipt_builder: Arc<signed_receipt::SignedReceiptBuilder>,
    pub jurisdiction_mapper: Arc<jurisdiction_mapper::JurisdictionMapper>,
    pub scitt_anchor: Arc<scitt_anchoring::SCITTAnchoringService>,
}

impl AATEngine {
    pub fn new() -> Self {
        Self {
            formatter: Arc::new(aat_formatter::AATFormatter::new()),
            receipt_builder: Arc::new(signed_receipt::SignedReceiptBuilder::new()),
            jurisdiction_mapper: Arc::new(jurisdiction_mapper::JurisdictionMapper::new()),
            scitt_anchor: Arc::new(scitt_anchoring::SCITTAnchoringService::new()),
        }
    }
}
