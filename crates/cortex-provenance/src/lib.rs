pub mod tracecaps;
pub mod merkle_chain;
pub mod vap_compliance;
pub mod scitt_builder;
pub mod field_level_audit;
pub mod continuous_evidence_chain;
pub mod aat_formatter;
pub mod signing;
pub mod audit_log;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top-level provenance orchestrator.
pub struct ProvenanceEngine {
    pub accumulator: tracecaps::TraceCapsAccumulator,
    pub merkle: merkle_chain::MerkleChainBuilder,
    pub vap: vap_compliance::VAPComplianceLayer,
    pub scitt: scitt_builder::SCITTReceiptBuilder,
    pub field_audit: field_level_audit::FieldLevelAuditTrail,
    pub evidence_chain: continuous_evidence_chain::ContinuousEvidenceChain,
    pub aat: aat_formatter::AATFormatter,
    pub signer: signing::Signer,
    pub ledger: Arc<RwLock<audit_log::AuditLog>>,
}

impl ProvenanceEngine {
    pub fn new(signing_key: [u8; 32]) -> Self {
        let signer = signing::Signer::new(signing_key);
        Self {
            accumulator: tracecaps::TraceCapsAccumulator::new(),
            merkle: merkle_chain::MerkleChainBuilder::new(),
            vap: vap_compliance::VAPComplianceLayer::new(),
            scitt: scitt_builder::SCITTReceiptBuilder::new(),
            field_audit: field_level_audit::FieldLevelAuditTrail::new(),
            evidence_chain: continuous_evidence_chain::ContinuousEvidenceChain::new(),
            aat: aat_formatter::AATFormatter::new(),
            signer,
            ledger: Arc::new(RwLock::new(audit_log::AuditLog::new())),
        }
    }
}
