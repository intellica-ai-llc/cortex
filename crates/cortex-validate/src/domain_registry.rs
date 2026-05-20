//! Cortex Research Domain Registry — The twelve academic domains.
//!
//! Each domain maps to a specific Cortex crate and a set of
//! verifiable claims that are validated by one or more experiments.

use serde::{Deserialize, Serialize};

/// The twelve research domains of Intellecta Cortex.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum ResearchDomain {
    /// D1 — MCP Security & Governance
    MCPSecurity,
    /// D2 — Semantic Tool Routing
    SemanticRouting,
    /// D3 — Cryptographic Provenance
    CryptographicProvenance,
    /// D4 — Organisational Agent Architecture
    AgentArchitecture,
    /// D5 — Application Obsolescence Pipeline
    ApplicationObsolescence,
    /// D6 — Direct Backup Parsing (Vault)
    BackupParsing,
    /// D7 — CDC Mirror Engine
    CDCMirror,
    /// D8 — Deep Research Fabric
    DeepResearch,
    /// D9 — Convergent Reasoning
    ConvergentReasoning,
    /// D10 — Multi-Modal Wellness
    MultiModalWellness,
    /// D11 — Generative UI (A2UI/AG-UI)
    GenerativeUI,
    /// D12 — Mobile/Edge AI
    MobileAI,
}

/// Metadata for a single research domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainMeta {
    pub domain: ResearchDomain,
    pub name: &'static str,
    pub cortex_crates: &'static [&'static str],
    pub key_claim: &'static str,
    pub experiment_ids: &'static [&'static str],
}

pub struct DomainRegistry {
    domains: Vec<DomainMeta>,
}

impl DomainRegistry {
    pub fn new() -> Self {
        Self {
            domains: vec![
                DomainMeta {
                    domain: ResearchDomain::MCPSecurity,
                    name: "MCP Security & Governance",
                    cortex_crates: &["cortex-security", "cortex-guard", "cortex-gateway"],
                    key_claim: "7-layer defence-in-depth neutralises all OWASP MCP Top 10 risk categories",
                    experiment_ids: &["mcp-security-x1"],
                },
                DomainMeta {
                    domain: ResearchDomain::SemanticRouting,
                    name: "Semantic Tool Routing",
                    cortex_crates: &["cortex-gateway"],
                    key_claim: "ClawRouter reduces token costs ≥70%; Semantic Firewall catches 100% of prompt injection in 500K fuzzing sequences",
                    experiment_ids: &["semantic-routing-x2"],
                },
                DomainMeta {
                    domain: ResearchDomain::CryptographicProvenance,
                    name: "Cryptographic Provenance",
                    cortex_crates: &["cortex-provenance"],
                    key_claim: "1M capsules remain Merkle-verifiable; SCITT-anchored receipts satisfy EU AI Act Art.12 & NERC CIP-015-1",
                    experiment_ids: &["provenance-integrity-x3"],
                },
                DomainMeta {
                    domain: ResearchDomain::AgentArchitecture,
                    name: "Organisational Agent Architecture",
                    cortex_crates: &["cortex-council"],
                    key_claim: "OMC E²R tree search achieves 84.67% PRDBench, +15.48pp over SOTA",
                    experiment_ids: &["agent-council-x4"],
                },
                DomainMeta {
                    domain: ResearchDomain::ApplicationObsolescence,
                    name: "Application Obsolescence Pipeline",
                    cortex_crates: &["cortex-absorb", "cortex-genesis", "cortex-replace", "cortex-retire"],
                    key_claim: "Six-phase pipeline absorbs ≥80% of legacy workflows within 4-6 weeks; Strangler Fig façade keeps users unaware",
                    experiment_ids: &["absorption-equivalence-x5"],
                },
                DomainMeta {
                    domain: ResearchDomain::BackupParsing,
                    name: "Direct Backup Parsing (Vault)",
                    cortex_crates: &["cortex-vault"],
                    key_claim: "Direct .bak/.dbf/IXF parsing achieves ≥99.99% checksum match without database instance",
                    experiment_ids: &["backup-extraction-x6"],
                },
                DomainMeta {
                    domain: ResearchDomain::CDCMirror,
                    name: "CDC Mirror Engine",
                    cortex_crates: &["cortex-mirror"],
                    key_claim: "Kafka-free direct CDC sustains 250M+ events/week at sub-100ms latency with guaranteed integrity",
                    experiment_ids: &["cdc-latency-x7"],
                },
                DomainMeta {
                    domain: ResearchDomain::DeepResearch,
                    name: "Deep Research Fabric",
                    cortex_crates: &["cortex-deep-research", "cortex-coggen", "cortex-iter-research"],
                    key_claim: "OpenSeeker-v2 SFT-only surpasses CPT+SFT+RL; IterResearch 2048+ tool calls at 40K context",
                    experiment_ids: &["deep-research-x8"],
                },
                DomainMeta {
                    domain: ResearchDomain::ConvergentReasoning,
                    name: "Convergent Reasoning",
                    cortex_crates: &["cortex-converge"],
                    key_claim: "Three-path convergent reasoning achieves higher factual accuracy than single-model inference",
                    experiment_ids: &["convergent-reasoning-x9"],
                },
                DomainMeta {
                    domain: ResearchDomain::MultiModalWellness,
                    name: "Multi-Modal Wellness",
                    cortex_crates: &["cortex-pulse", "cortex-whisper"],
                    key_claim: "Voice+eye Bayesian fusion clinically validated; burnout early warning detects signals 11 days before self-report",
                    experiment_ids: &["wellness-correlation-x10"],
                },
                DomainMeta {
                    domain: ResearchDomain::GenerativeUI,
                    name: "Generative UI (A2UI/AG-UI)",
                    cortex_crates: &["cortex-interface", "cortex-genesis"],
                    key_claim: "18-component A2UI v0.9 catalog with WCAG 2.1 AA; UX Middleware eliminates hallucinated UI",
                    experiment_ids: &["genui-compliance-x11"],
                },
                DomainMeta {
                    domain: ResearchDomain::MobileAI,
                    name: "Mobile/Edge AI",
                    cortex_crates: &["cortex-mobile", "lfab-core", "lfab-sleep"],
                    key_claim: "LFAB S-HAI Core on 4GB phone; CRDT sync conflict-free; hierarchical controller routes tasks optimally",
                    experiment_ids: &["mobile-parity-x12"],
                },
            ],
        }
    }

    /// Get all domain metadata.
    pub fn all(&self) -> &[DomainMeta] { &self.domains }

    /// Look up a domain by enum.
    pub fn get(&self, domain: &ResearchDomain) -> Option<&DomainMeta> {
        self.domains.iter().find(|d| &d.domain == domain)
    }
}
