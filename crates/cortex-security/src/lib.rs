//! Cortex SecurityFortress — seven-layer defence-in-depth.
//!
//! Based on MCP-DPT (Rostamzadeh et al., 2026): 6-layer taxonomy
//! plus a seventh formal verification layer (Peyrano 500K fuzzer).
//!
//! Layers:
//!   1. Semantic Firewall    — pre-inference filtering (Peyrano L1)
//!   2. Tool-Level RBAC      — deterministic access control (Peyrano L2)
//!   3. Crypto HITL          — out-of-band approval (Peyrano L3)
//!   4. CABP Pipeline        — 6-stage identity (Srinivasan)
//!   5. MCPShield Cognition  — three-phase probe-execute-reflect (Zhou)
//!   6. MCIP Integrity       — contextual integrity checks
//!   7. Fuzzing Engine       — greybox semantic fuzzer (Peyrano)
//!
//! Extended modules:
//!   - AGT Policy Engine     — Microsoft Agent Governance Toolkit bridge
//!   - Shadow MCP Detector   — unauthorised server detection
//!   - OWASP Compliance      — MCP Top 10 + Agentic Top 10 mapping
//!   - OAuth Lifecycle       — token scope, TTL, auto-revocation
//!   - MCP Sandbox           — STDIO isolation via microVM
//!   - SERF Envelope         — Structured Error Recovery Framework

pub mod semantic_firewall;
pub mod tool_rbac;
pub mod crypto_hitl;
pub mod cabp_pipeline;
pub mod mcpshield_cognition;
pub mod mcip_checks;
pub mod fuzzing_engine;
pub mod agt_policy_engine;
pub mod shadow_mcp_detector;
pub mod owasp_compliance;
pub mod oauth;
pub mod oauth_lifecycle;
pub mod mcp_sandbox;
pub mod serf_envelope;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top-level security orchestrator.
pub struct SecurityFortress {
    pub firewall: semantic_firewall::SemanticFirewall,
    pub rbac: Arc<RwLock<tool_rbac::ToolLevelRBAC>>,
    pub hitl: crypto_hitl::CryptoHITL,
    pub cabp: cabp_pipeline::CABPPipeline,
    pub cognition: mcpshield_cognition::MCPShieldCognition,
    pub mcip: mcip_checks::MCIPIntegrity,
    pub fuzzer: fuzzing_engine::FuzzingEngine,
    pub agt: agt_policy_engine::AGTPolicyEngine,
    pub shadow: shadow_mcp_detector::ShadowMCPDetector,
    pub owasp: owasp_compliance::OWASPCompliance,
    pub oauth_lifecycle: oauth_lifecycle::OAuthLifecycle,
    pub sandbox: mcp_sandbox::MCPSandbox,
}

impl SecurityFortress {
    pub fn new() -> Self {
        Self {
            firewall: semantic_firewall::SemanticFirewall::new(),
            rbac: Arc::new(RwLock::new(tool_rbac::ToolLevelRBAC::new())),
            hitl: crypto_hitl::CryptoHITL::new(),
            cabp: cabp_pipeline::CABPPipeline::new(),
            cognition: mcpshield_cognition::MCPShieldCognition::new(),
            mcip: mcip_checks::MCIPIntegrity::new(),
            fuzzer: fuzzing_engine::FuzzingEngine::new(),
            agt: agt_policy_engine::AGTPolicyEngine::new(),
            shadow: shadow_mcp_detector::ShadowMCPDetector::new(),
            owasp: owasp_compliance::OWASPCompliance::new(),
            oauth_lifecycle: oauth_lifecycle::OAuthLifecycle::new(),
            sandbox: mcp_sandbox::MCPSandbox::new(),
        }
    }

    /// Full validation of a tool call across all layers.
    pub async fn validate_tool_call(
        &self,
        intent: &str,
        tool: &str,
        user: &str,
        params: &serde_json::Value,
    ) -> Result<SecurityVerdict, SecurityError> {
        // Layer 1: Semantic Firewall
        self.firewall.evaluate(intent, tool, params)?;

        // Layer 2: Tool-Level RBAC
        let rbac = self.rbac.read().await;
        rbac.authorize(user, tool, params)?;

        // Layer 3: Cryptographic HITL (if risk threshold exceeded)
        let risk = self.firewall.risk_score(intent, tool);
        if risk > 0.7 {
            self.hitl.request_approval(user, tool, params).await?;
        }

        // Layer 4: CABP Pipeline
        self.cabp.validate_identity(user).await?;

        // Layer 5: MCPShield Cognition
        self.cognition.probe(tool).await?;

        // Layer 6: MCIP Integrity
        self.mcip.check_context(intent, user, tool)?;

        Ok(SecurityVerdict::Approved {
            risk,
            layers_cleared: 6,
            audit_id: uuid::Uuid::new_v4(),
        })
    }
}

#[derive(Debug)]
pub enum SecurityVerdict {
    Approved { risk: f64, layers_cleared: u8, audit_id: uuid::Uuid },
    Rejected { reason: String, layer_failed: u8 },
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Firewall rejected: {0}")]
    FirewallRejected(String),
    #[error("RBAC denied: {0}")]
    RBACDenied(String),
    #[error("HITL not approved: {0}")]
    HITLNotApproved(String),
    #[error("Identity verification failed: {0}")]
    IdentityFailed(String),
    #[error("Cognition probe failed: {0}")]
    CognitionFailed(String),
    #[error("Contextual integrity violation: {0}")]
    ContextViolation(String),
}
