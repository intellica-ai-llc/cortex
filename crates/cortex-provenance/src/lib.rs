pub mod tracecaps;
pub mod merkle_chain;
pub mod vap_compliance;
pub mod scitt_builder;
pub mod signing;
pub mod audit_log;
pub mod continuous_evidence_chain;
pub mod aat_formatter;
pub mod field_level_audit;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ProvenanceEngine {
    pub accumulator: Arc<RwLock<tracecaps::TraceCapsAccumulator>>,
    pub merkle: merkle_chain::MerkleChainBuilder,
    pub vap: vap_compliance::VAPComplianceLayer,
    pub scitt: scitt_builder::SCITTReceiptBuilder,
    pub signer: signing::Signer,
    pub ledger: Arc<RwLock<audit_log::AuditLog>>,
}

impl ProvenanceEngine {
    pub fn new(signing_key: [u8; 32]) -> Self {
        let signer = signing::Signer::new(signing_key);
        Self {
            accumulator: Arc::new(RwLock::new(tracecaps::TraceCapsAccumulator::new())),
            merkle: merkle_chain::MerkleChainBuilder::new(),
            vap: vap_compliance::VAPComplianceLayer::new(),
            scitt: scitt_builder::SCITTReceiptBuilder::new(),
            signer,
            ledger: Arc::new(RwLock::new(audit_log::AuditLog::new())),
        }
    }
}