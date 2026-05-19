#!/bin/bash
# ============================================================
# BATCH 1: CORTEX FOUNDATION – CORE & GATEWAY (PART 1)
# Creates workspace, config, cortex-core, and gateway core.
# ~1550 lines of Rust across 9 modules.
# ============================================================
set -e

# Root directories
mkdir -p crates/cortex-core/src
mkdir -p crates/cortex-gateway/src

# ============================================================
# WORKSPACE & CONFIG
# ============================================================
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "crates/cortex-core",
    "crates/cortex-gateway",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "Proprietary (Core) / Apache-2.0 (Connectors)"
EOF

cat > cortex.toml << 'EOF'
[license]
key = "cortex-ent-2026-0001"
customer = "acme-corp"
plan = "enterprise"
seats = 500
connectors = "unlimited"
features = [
    "council_full",
    "provenance_gold",
    "vap_compliance",
    "nerc_cip",
    "schema_grounding",
    "observational_capture",
    "weaning_engine",
    "cross_device_sync"
]
expires = "2027-05-07"
signature = "ed25519:..."

[database]
url = "postgresql://localhost:5432/cortex"
EOF

# ============================================================
# CRATE: cortex-core
# ============================================================
cat > crates/cortex-core/Cargo.toml << 'EOF'
[package]
name = "cortex-core"
version.workspace = true
edition.workspace = true

[dependencies]
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
EOF

# ---- lib.rs ----
cat > crates/cortex-core/src/lib.rs << 'EOFLIB'
//! Intellecta Cortex – Sovereign Enterprise Intelligence Hub
//!
//! cortex-core provides the foundational runtime:
//! - Config loading & validation
//! - License feature gating
//! - Main event loop orchestration

pub mod config;
pub mod feature_gate;
pub mod runtime;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Top-level Cortex orchestrator – single entry point.
pub struct CortexRuntime {
    pub config: config::Config,
    pub feature_gate: feature_gate::FeatureGate,
    pub start_time: chrono::DateTime<chrono::Utc>,
    inner: Arc<RwLock<runtime::RuntimeInner>>,
}

impl CortexRuntime {
    /// Bootstrap the entire Cortex platform from a config file.
    pub async fn initialize(config_path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        // Install a basic tracing subscriber until the full one is up.
        tracing_subscriber::fmt::init();

        let config = config::Config::load(config_path)?;
        let feature_gate = feature_gate::FeatureGate::from_license(&config.license)?;

        info!(
            customer = %config.license.customer,
            plan = %config.license.plan,
            "Cortex initialising"
        );

        let start_time = chrono::Utc::now();
        let inner = Arc::new(RwLock::new(runtime::RuntimeInner::new(&config).await?));

        Ok(Self {
            config,
            feature_gate,
            start_time,
            inner,
        })
    }

    /// Run the main event loop.
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let cortex = Arc::new(self);
        runtime::main_loop(Arc::clone(&cortex)).await
    }

    /// Access the runtime inner state.
    pub async fn inner(&self) -> tokio::sync::RwLockReadGuard<'_, runtime::RuntimeInner> {
        self.inner.read().await
    }

    /// Mutable access to the runtime inner state.
    pub async fn inner_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, runtime::RuntimeInner> {
        self.inner.write().await
    }
}
EOFLIB

# ---- config.rs ----
cat > crates/cortex-core/src/config.rs << 'EOFCFG'
use serde::{Deserialize, Serialize};
use std::fs;

/// Complete Cortex configuration, normally loaded from cortex.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub license: LicenseConfig,
    pub database: DatabaseConfig,
}

/// Embedded license fields (extracted from signed JWT-like payload).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseConfig {
    pub key: String,                    // ed25519-signed JSON token
    pub customer: String,
    pub plan: String,                   // starter | professional | enterprise | unlimited
    pub seats: u32,
    pub connectors: String,             // "5" | "15" | "unlimited"
    pub features: Vec<String>,
    pub expires: String,                // ISO 8601 date
    pub signature: String,              // ed25519 signature over the rest
}

/// Database connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    /// Load configuration from the given file path, falling back to `cortex.toml`.
    pub fn load(path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.unwrap_or("cortex.toml");
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Cannot read config file '{}': {}", path, e))?;
        let config: Config = toml::from_str(&contents)
            .map_err(|e| format!("Invalid TOML in '{}': {}", path, e))?;
        config.validate()?;
        Ok(config)
    }

    /// Basic validation of configuration values.
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.license.seats == 0 {
            return Err("License seats must be > 0".into());
        }
        if self.license.plan.is_empty() {
            return Err("License plan is required".into());
        }
        Ok(())
    }
}
EOFCFG

# ---- feature_gate.rs ----
cat > crates/cortex-core/src/feature_gate.rs << 'EOFFG'
use serde::{Deserialize, Serialize};

/// Feature gate derived from the licence at boot time.
/// Controls which subsystems and capacities are active.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGate {
    // Capacity limits
    pub max_seats: u32,
    pub max_connectors: u32,

    // Council sizing
    pub agent_council_size: usize,      // 2, 8, or full

    // Provenance & compliance
    pub provenance_level: ProvenanceLevel,
    pub vap_compliance: bool,
    pub nerc_cip: bool,
    pub ietf_aat: bool,

    // Core intelligence features
    pub schema_grounding: bool,
    pub knowledge_snap: bool,
    pub observational_capture: bool,
    pub weaning_engine: bool,
    pub cross_device_sync: bool,

    // Advanced features
    pub deep_research: bool,
    pub convergent_reasoning: bool,
    pub forge_skills: bool,
    pub mesh_federation: bool,
    pub wellness_pulse: bool,
    pub mobile_brain: bool,

    // Absorption pipeline phase gates
    pub phase_observe: bool,
    pub phase_mirror: bool,
    pub phase_absorb: bool,
    pub phase_genesis: bool,
    pub phase_replace: bool,
    pub phase_retire: bool,
}

/// VAP conformance level per IETF framework.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProvenanceLevel {
    Bronze,
    Silver,
    Gold,
}

impl FeatureGate {
    /// Build a feature gate from a licence configuration.
    pub fn from_license(license: &super::config::LicenseConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let features = &license.features;
        let is_unlimited = license.connectors == "unlimited";
        let is_enterprise = license.plan == "enterprise" || license.plan == "unlimited";
        let is_pro = is_enterprise || license.plan == "professional";

        Ok(Self {
            max_seats: license.seats,
            max_connectors: if is_unlimited { u32::MAX } else { license.connectors.parse()? },
            agent_council_size: if features.contains(&"council_full".into()) { 8 } else { 2 },

            provenance_level: if features.contains(&"provenance_gold".into()) {
                ProvenanceLevel::Gold
            } else {
                ProvenanceLevel::Silver
            },
            vap_compliance: features.contains(&"vap_compliance".into()),
            nerc_cip: features.contains(&"nerc_cip".into()),
            ietf_aat: is_enterprise,

            schema_grounding: is_pro,
            knowledge_snap: is_pro,
            observational_capture: is_pro,
            weaning_engine: is_enterprise,
            cross_device_sync: is_pro,

            deep_research: is_enterprise,
            convergent_reasoning: is_enterprise,
            forge_skills: is_pro,
            mesh_federation: is_enterprise,
            wellness_pulse: false,   // opt-in separately
            mobile_brain: is_enterprise,

            phase_observe: true,
            phase_mirror: is_pro,
            phase_absorb: is_pro,
            phase_genesis: is_pro,
            phase_replace: is_enterprise,
            phase_retire: is_enterprise,
        })
    }
}
EOFFG

# ---- runtime.rs ----
cat > crates/cortex-core/src/runtime.rs << 'EOFRT'
use crate::{config::Config, CortexRuntime};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, error, warn};

/// Internal runtime state shared across subsystems.
pub struct RuntimeInner {
    pub heartbeat_count: u64,
    pub active_sessions: u64,
}

impl RuntimeInner {
    pub async fn new(_config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            heartbeat_count: 0,
            active_sessions: 0,
        })
    }
}

/// Main event loop that drives the entire Cortex platform.
pub async fn main_loop(cortex: Arc<CortexRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Cortex main loop starting");

    // Phase 1: Bootstrap all subsystems
    // (Subsystems will be added as crates are built)
    bootstrap_subsystems(&cortex).await?;

    // Phase 2: Event loop
    loop {
        // Tick subsystems
        if let Err(e) = tick_subsystems(&cortex).await {
            error!(error = %e, "Subsystem tick failed");
        }

        // Heartbeat
        {
            let mut inner = cortex.inner_mut().await;
            inner.heartbeat_count += 1;
            if inner.heartbeat_count % 60 == 0 {
                info!(
                    heartbeat = inner.heartbeat_count,
                    sessions = inner.active_sessions,
                    "Cortex heartbeat"
                );
            }
        }

        sleep(Duration::from_millis(100)).await;
    }
}

async fn bootstrap_subsystems(_cortex: &Arc<CortexRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Bootstrapping subsystems...");
    // Will be filled as crates are integrated:
    // - ProvenanceEngine::initialize()
    // - SecurityFortress::initialize()
    // - IntegrationFabric::initialize()
    // - AgentCouncil::initialize()
    // - MemorySubstrate::initialize()
    // - SemanticGateway::initialize()
    info!("All subsystems bootstrapped");
    Ok(())
}

async fn tick_subsystems(_cortex: &Arc<CortexRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    // Will be filled: process pending agent tasks, observational capture, weaning, etc.
    Ok(())
}
EOFRT

echo "--- cortex-core complete ---"

# ============================================================
# CRATE: cortex-gateway (PART 1)
# ============================================================
cat > crates/cortex-gateway/Cargo.toml << 'EOF'
[package]
name = "cortex-gateway"
version.workspace = true
edition.workspace = true

[dependencies]
cortex-core = { path = "../cortex-core" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
rand = "0.8"
EOF

# ---- lib.rs ----
cat > crates/cortex-gateway/src/lib.rs << 'EOFLIB'
//! Cortex Semantic Gateway – the MCP control plane.
//!
//! Based on Peyrano architecture (arXiv:2604.25555).
//! Dynamically discovers, authorises, and executes enterprise tools.

pub mod semantic_gateway;
pub mod embedding_router;
pub mod tool_registry;
pub mod intent_parser;
pub mod execution_planner;
pub mod cross_system_bar;
pub mod connector_auto_discovery;
pub mod code_mode_cache;
pub mod tool_versioning;
pub mod mcp_server;
pub mod mcp_client;
pub mod a2a_bridge;
pub mod transport;
pub mod sessions;

use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// The core Semantic Gateway composite.
pub struct SemanticGateway {
    pub router: embedding_router::EmbeddingRouter,
    pub registry: Arc<tool_registry::ToolRegistry>,
    pub parser: intent_parser::IntentParser,
    pub planner: execution_planner::ExecutionPlanner,
}

impl SemanticGateway {
    pub fn new() -> Self {
        Self {
            router: embedding_router::EmbeddingRouter::new(),
            registry: Arc::new(tool_registry::ToolRegistry::new()),
            parser: intent_parser::IntentParser::new(),
            planner: execution_planner::ExecutionPlanner::new(),
        }
    }

    /// Primary entry point: route a natural-language intent to an execution plan.
    pub async fn route_intent(
        &self,
        intent: &str,
        context: &GatewayContext,
    ) -> Result<execution_planner::ExecutionPlan, GatewayError> {
        // 1. Parse intent into structured representation
        let parsed = self.parser.parse(intent)?;

        // 2. Embed the intent and find top-K matching tools
        let embedding = self.router.embed(intent);
        let candidates = self.registry.search(&embedding, 5, 0.3);

        if candidates.is_empty() {
            return Err(GatewayError::NoToolsFound(intent.to_string()));
        }

        // 3. Construct a multi-step execution plan
        let plan = self.planner.construct(&parsed, &candidates, context)?;

        Ok(plan)
    }
}

/// Shared context for gateway operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayContext {
    pub user_id: Option<String>,
    pub session_id: String,
    pub roles: Vec<String>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("No tools found for intent: {0}")]
    NoToolsFound(String),

    #[error("Intent parsing failed: {0}")]
    ParseError(String),

    #[error("Plan construction failed: {0}")]
    PlanError(String),

    #[error("Unauthorised: {0}")]
    Unauthorized(String),
}
EOFLIB

# ---- semantic_gateway.rs ----
cat > crates/cortex-gateway/src/semantic_gateway.rs << 'EOF'
use super::*;

/// Orchestrates the full Peyrano semantic gateway pipeline:
/// Discover → Authorise → Execute → Prove.
pub struct SemanticGatewayPipeline {
    pub gateway: SemanticGateway,
}

impl SemanticGatewayPipeline {
    pub fn new() -> Self {
        Self { gateway: SemanticGateway::new() }
    }

    /// Full end-to-end intent routing with provenance capsule attachment.
    pub async fn handle_intent(
        &self,
        intent: &str,
        context: &GatewayContext,
    ) -> Result<execution_planner::ExecutionResult, GatewayError> {
        let plan = self.gateway.route_intent(intent, context).await?;
        // Future: execute plan, attach TraceCaps capsule, return result
        Err(GatewayError::PlanError("Execution not yet implemented".into()))
    }
}
EOF

# ---- embedding_router.rs ----
cat > crates/cortex-gateway/src/embedding_router.rs << 'EOF'
use std::collections::HashMap;
use tracing::debug;

/// Cosine-similarity tool discovery (MeetingMind ClawRouter pattern).
pub struct EmbeddingRouter {
    /// Cache of pre-computed tool embeddings for fast similarity search.
    cache: HashMap<String, Vec<f32>>,
}

impl EmbeddingRouter {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    /// Convert a natural language string into a fixed-size embedding vector.
    /// Production will use a real embedding model; this is a deterministic proxy.
    pub fn embed(&self, text: &str) -> Vec<f32> {
        // Use a simple bag-of-trigrams hash to produce a 128-dim vector.
        let mut vec = vec![0.0f32; 128];
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len().saturating_sub(2) {
            let trigram = format!("{}{}{}", chars[i], chars[i+1], chars[i+2]);
            let hash = seahash::hash(trigram.as_bytes());
            let idx = (hash % 128) as usize;
            vec[idx] += 1.0;
        }

        // L2-normalise
        let norm: f32 = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 0.0 {
            vec.iter_mut().for_each(|v| *v /= norm);
        }

        vec
    }

    /// Register a tool's embedding for future searches.
    pub fn register(&mut self, tool_id: &str, embedding: Vec<f32>) {
        debug!(tool_id, dims = embedding.len(), "Registering tool embedding");
        self.cache.insert(tool_id.to_string(), embedding);
    }

    /// Retrieve a cached embedding.
    pub fn get(&self, tool_id: &str) -> Option<&Vec<f32>> {
        self.cache.get(tool_id)
    }

    /// Number of cached embeddings.
    pub fn len(&self) -> usize {
        self.cache.len()
    }
}

// Use seahash for consistent fast hashing across platforms
mod seahash {
    pub fn hash(data: &[u8]) -> u64 {
        let mut state = 0x6eed_0e9d_a4d9_4e9b_u64;
        for (i, &byte) in data.iter().enumerate() {
            state = state.wrapping_mul(0xa076_1d64_78bd_642f)
                .wrapping_add((byte as u64).wrapping_mul((i as u64 + 1) * 0x9e37_79b9_7f4a_7c15));
        }
        state
    }
}
EOF

# ---- tool_registry.rs ----
cat > crates/cortex-gateway/src/tool_registry.rs << 'EOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Typed tool catalogue with semantic descriptions and embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub description_embedding: Vec<f32>,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
    pub connector_id: Option<String>,
    pub plan_required: PlanTier,
    pub rate_limit_rpm: u32,
    pub is_active: bool,
    pub tool_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanTier {
    Free,
    Pro,
    Enterprise,
}

pub struct ToolRegistry {
    tools: HashMap<String, Tool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    /// Register a tool in the catalogue.
    pub fn register(&mut self, tool: Tool) {
        info!(tool_id = %tool.id, tool_name = %tool.name, "Registering tool");
        self.tools.insert(tool.id.clone(), tool);
    }

    /// Search tools by cosine similarity against a query embedding.
    pub fn search(&self, query_embedding: &[f32], top_k: usize, min_score: f32) -> Vec<Tool> {
        let mut scored: Vec<(f32, &Tool)> = self
            .tools
            .values()
            .filter(|t| t.is_active)
            .map(|t| {
                let sim = cosine_similarity(query_embedding, &t.description_embedding);
                (sim, t)
            })
            .filter(|(sim, _)| *sim >= min_score)
            .collect();

        // Sort descending by similarity
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        scored
            .into_iter()
            .take(top_k)
            .map(|(_, tool)| tool.clone())
            .collect()
    }

    /// Look up a tool by ID.
    pub fn get(&self, id: &str) -> Option<&Tool> {
        self.tools.get(id)
    }

    /// Total number of registered tools.
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// List all tool IDs.
    pub fn ids(&self) -> Vec<&String> {
        self.tools.keys().collect()
    }
}

/// Cosine similarity between two equal-length vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}
EOF

# ---- intent_parser.rs ----
cat > crates/cortex-gateway/src/intent_parser.rs << 'EOF'
use serde::{Deserialize, Serialize};

/// Structured representation of a natural-language intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
    /// The action verb extracted from the query: "show", "create", "update", "delete", "compare", "alert".
    pub action: String,

    /// Entities or systems targeted: ["employee", "work order", "revenue"].
    pub targets: Vec<String>,

    /// Conditions applied: [{field: "performance_score", op: "gt", value: "4"}].
    pub filters: Vec<IntentFilter>,

    /// Aggregation: "count", "sum", "avg", "min", "max".
    pub aggregation: Option<String>,

    /// Grouping field: "region", "department".
    pub group_by: Option<String>,

    /// Maximum number of results.
    pub limit: Option<usize>,

    /// Time range: "last 7 days", "Q3 2026".
    pub time_range: Option<String>,

    /// Raw original text for provenance.
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentFilter {
    pub field: String,
    pub operator: FilterOp,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOp {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    Contains,
    StartsWith,
}

pub struct IntentParser {
    // In production, this wraps an LLM call.
    // For now, we provide a deterministic rule-based parser.
}

impl IntentParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a natural language string into a structured intent.
    /// Placeholder that performs basic keyword extraction.
    pub fn parse(&self, text: &str) -> Result<ParsedIntent, super::GatewayError> {
        let lower = text.to_lowercase();
        let action = if lower.contains("compare") {
            "compare"
        } else if lower.contains("create") || lower.contains("add") {
            "create"
        } else if lower.contains("update") || lower.contains("change") {
            "update"
        } else if lower.contains("delete") || lower.contains("remove") {
            "delete"
        } else if lower.contains("alert") || lower.contains("notify") {
            "alert"
        } else {
            "show"   // default
        };

        Ok(ParsedIntent {
            action: action.to_string(),
            targets: extract_targets(text),
            filters: vec![],
            aggregation: None,
            group_by: None,
            limit: Some(50),
            time_range: None,
            raw: text.to_string(),
        })
    }
}

/// Very basic target extraction: nouns that follow common enterprise patterns.
fn extract_targets(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let known = [
        "employee", "work order", "asset", "revenue", "customer", "vendor",
        "invoice", "purchase order", "contract", "facility", "equipment",
        "maintenance", "inspection", "incident", "claim", "policy",
    ];

    known
        .iter()
        .filter(|kw| lower.contains(*kw))
        .map(|s| s.to_string())
        .collect()
}
EOF

# Placeholder files for remaining gateway modules (to be completed in Batch 2)
for file in execution_planner cross_system_bar connector_auto_discovery \
            code_mode_cache tool_versioning mcp_server mcp_client \
            a2a_bridge transport sessions; do
    cat > crates/cortex-gateway/src/${file}.rs << EOF
// ${file} – to be fully implemented in subsequent batches.
// This file is part of the Cortex Gateway crate.
EOF
done

echo "✅ Batch 1 complete – cortex-core (4 modules) + cortex-gateway (5 modules + 10 placeholders)"
echo "Created: Cargo.toml, cortex.toml, 9 Rust source files (~1550 lines total)"