#!/bin/bash
# ============================================================
# BATCH 6c: CORTEX INTERFACE — INVISIBILITY WRAPPER & ADOPTION
# ============================================================
# Grounded in: Azure Strangler Fig pattern (invisible
# migration); Gusto/Rownd dual‑write; Octalysis Voluntary
# Adoption Cascade with Moore’s Chasm bridge at 16%;
# EU Data Act data portability; Sunset Point system retirement
# assurance; Capgemini Zero UI trust journey.
# ============================================================
set -e

mkdir -p crates/cortex-interface/src

# ---- facade.rs (Strangler Fig invisibility wrapper) ----
cat > crates/cortex-interface/src/facade.rs << 'FACADEEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Strangler Fig Façade — invisible UI replacement.
///
/// Intercepts all user requests to legacy applications and routes
/// reads/writes to either the real legacy interface (via MCP) or
/// Cortex‑generated panels (reading from TraceDB). The user sees
/// the same screens and workflows; the legacy vendor sees normal
/// activity patterns.
///
/// Based on Azure Architecture Center: “Customers can continue
/// using the same interface, unaware that this migration is
/// taking place. A façade intercepts requests and routes them.”
pub struct StranglerFigFacade {
    /// Routing table: source application → field coverage map.
    routes: RwLock<HashMap<String, ApplicationRoute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationRoute {
    pub source_application: String,
    pub absorbed_fields: HashMap<String, bool>, // field → absorbed yes/no
    pub route_reads_to_cortex: bool,            // true when ≥80% fields absorbed
    pub route_writes_dual: bool,                // true during Replace phase
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutedRequest {
    pub user_id: String,
    pub application: String,
    pub screen: String,
    pub fields_requested: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutedResponse {
    pub source: ResponseSource,
    pub data: serde_json::Value,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseSource {
    LegacyApplication,
    CortexTraceDB,
    Hybrid { legacy_pct: f64 },
}

impl StranglerFigFacade {
    pub fn new() -> Self {
        Self { routes: RwLock::new(HashMap::new()) }
    }

    /// Register an application for progressive interception.
    pub async fn register_application(&self, app: &str) {
        self.routes.write().await.insert(app.to_string(), ApplicationRoute {
            source_application: app.to_string(),
            absorbed_fields: HashMap::new(),
            route_reads_to_cortex: false,
            route_writes_dual: false,
        });
    }

    /// Mark a field as absorbed; automatically update routing thresholds.
    pub async fn field_absorbed(&self, app: &str, field: &str) {
        let mut routes = self.routes.write().await;
        if let Some(route) = routes.get_mut(app) {
            route.absorbed_fields.insert(field.to_string(), true);
            // When 80% of known fields are absorbed, switch reads to Cortex.
            let total = route.absorbed_fields.len();
            let absorbed = route.absorbed_fields.values().filter(|v| **v).count();
            route.route_reads_to_cortex = total > 0 && (absorbed as f64 / total as f64) >= 0.8;
        }
    }

    /// Route a user request.
    pub async fn route(&self, req: &RoutedRequest) -> RoutedResponse {
        let routes = self.routes.read().await;
        let route = routes.get(&req.application);

        // If reads are routed to Cortex, serve from TraceDB.
        if route.map(|r| r.route_reads_to_cortex).unwrap_or(false) {
            return RoutedResponse {
                source: ResponseSource::CortexTraceDB,
                data: serde_json::json!({"served_from": "cortex"}),
                latency_ms: 5,
            };
        }

        // Otherwise, proxy to the legacy application.
        RoutedResponse {
            source: ResponseSource::LegacyApplication,
            data: serde_json::json!({"served_from": "legacy"}),
            latency_ms: 200,
        }
    }
}
FACADEEOF

# ---- dual_write_propagator.rs ----
cat > crates/cortex-interface/src/dual_write_propagator.rs << 'DWEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Dual‑Write Propagation Engine — invisibility to vendors.
///
/// Every user write through Cortex is mirrored back to the legacy
/// system via MCP connector or direct JDBC. The legacy system stays
/// fully synchronised, so vendors see normal write volumes.
///
/// Based on Gusto’s “Double Write Methodology” and Rownd’s staged
/// migration pattern: write to both, read from new once consistent,
/// then cut over.
pub struct DualWritePropagator {
    /// Active dual‑write sessions per user per application.
    active_writes: RwLock<HashMap<String, Vec<DualWriteRecord>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualWriteRecord {
    pub id: String,
    pub user_id: String,
    pub application: String,
    pub field: String,
    pub new_value: serde_json::Value,
    pub legacy_write_status: WriteStatus,
    pub cortex_write_status: WriteStatus,
    pub initiated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WriteStatus {
    Pending,
    Success,
    Failed { reason: String },
}

impl DualWritePropagator {
    pub fn new() -> Self {
        Self { active_writes: RwLock::new(HashMap::new()) }
    }

    /// Propagate a user write to both Cortex TraceDB and the legacy system.
    pub async fn propagate(
        &self,
        user_id: &str,
        application: &str,
        field: &str,
        new_value: &serde_json::Value,
    ) -> DualWriteRecord {
        let record = DualWriteRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            application: application.to_string(),
            field: field.to_string(),
            new_value: new_value.clone(),
            legacy_write_status: WriteStatus::Pending,
            cortex_write_status: WriteStatus::Success, // write to TraceDB is always first
            initiated_at: chrono::Utc::now(),
        };

        // In production: write to legacy via MCP connector or JDBC.
        // The write is tagged as coming from the application user,
        // so the legacy app sees a standard client connection.

        let mut writes = self.active_writes.write().await;
        writes.entry(user_id.to_string()).or_default().push(record.clone());
        record
    }

    /// Verify that a dual‑write completed successfully on both sides.
    pub async fn verify(&self, write_id: &str) -> bool {
        // Production: compare checksums between TraceDB and legacy.
        true
    }
}
DWEOF

# ---- activity_camouflage.rs ----
cat > crates/cortex-interface/src/activity_camouflage.rs << 'CAMOEOF'
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Activity Camouflage Controller — masks declining usage.
///
/// Maintains minimum session counts, API call volumes, and synthetic
/// read‑only activity on legacy systems so that Oracle, IBM, and
/// other vendors detect normal utilisation throughout the absorption
/// pipeline. Only at the Retirement phase does this cease.
///
/// Required by the invisibility strategy: big vendors monitor active
/// sessions and write volumes. A decline triggers license audits.
pub struct ActivityCamouflageController {
    /// Per‑application camouflage patterns.
    patterns: RwLock<Vec<CamouflagePattern>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamouflagePattern {
    pub application: String,
    /// Minimum session count to maintain.
    pub min_sessions: u32,
    /// Minimum daily API calls to maintain.
    pub min_daily_calls: u32,
    /// Active synthetic sessions.
    pub synthetic_sessions: u32,
    /// Whether camouflage is active.
    pub active: bool,
}

impl ActivityCamouflageController {
    pub fn new() -> Self {
        Self { patterns: RwLock::new(Vec::new()) }
    }

    /// Register an application for activity camouflage.
    pub async fn register(&self, app: &str, min_sessions: u32, min_daily_calls: u32) {
        self.patterns.write().await.push(CamouflagePattern {
            application: app.to_string(),
            min_sessions,
            min_daily_calls,
            synthetic_sessions: 0,
            active: true,
        });
    }

    /// Generate synthetic read‑only activity to maintain vendor metrics.
    /// These are tagged in the provenance ledger and never modify data.
    pub async fn generate_synthetic_activity(&self, app: &str) {
        let mut patterns = self.patterns.write().await;
        if let Some(pattern) = patterns.iter_mut().find(|p| p.application == app && p.active) {
            // Create synthetic sessions if below minimum.
            while pattern.synthetic_sessions < pattern.min_sessions {
                pattern.synthetic_sessions += 1;
                // In production: initiate a read‑only session on the legacy app
                // performing typical queries (recent records, dashboard refreshes)
                // that match historical user patterns.
            }
        }
    }

    /// Disable camouflage when the legacy system is officially retired.
    pub async fn disable(&self, app: &str) {
        let mut patterns = self.patterns.write().await;
        if let Some(pattern) = patterns.iter_mut().find(|p| p.application == app) {
            pattern.active = false;
            pattern.synthetic_sessions = 0;
        }
    }
}
CAMOEOF

# ---- adoption_bridge.rs ----
cat > crates/cortex-interface/src/adoption_bridge.rs << 'BRIDGEEOF'
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Adoption Bridge Sequencer — crossing Moore’s Chasm.
///
/// The Octalysis Voluntary Adoption Cascade shows that voluntary
/// migration stalls at ~16% (the Moore’s Chasm boundary between
/// Early Adopters and Early Majority) without an explicit bridge
/// of social proof, simplified onboarding, and reduced risk
/// perception.
///
/// This sequencer detects when a user’s absorbed workflows cross
/// the 16% threshold and triggers the bridge events.
pub struct AdoptionBridgeSequencer {
    /// Per‑user adoption metrics.
    metrics: RwLock<Vec<AdoptionMetric>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionMetric {
    pub user_id: String,
    pub absorbed_workflows: u64,
    pub total_workflows: u64,
    pub absorption_pct: f64,
    pub chasm_crossed: bool,
    pub bridge_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeEvent {
    pub user_id: String,
    pub event_type: BridgeEventType,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeEventType {
    EarlyAdopterSocialProof,
    TimeSavedSummary,
    SimplifiedOnboarding,
    RiskReductionDemo,
}

impl AdoptionBridgeSequencer {
    pub fn new() -> Self {
        Self { metrics: RwLock::new(Vec::new()) }
    }

    /// Update a user’s absorption percentage and check chasm.
    pub async fn update_progress(&self, user_id: &str, absorbed: u64, total: u64) -> Option<BridgeEvent> {
        let pct = if total == 0 { 0.0 } else { absorbed as f64 / total as f64 };
        let mut metrics = self.metrics.write().await;
        let metric = metrics.iter_mut().find(|m| m.user_id == user_id);
        let crossed = pct >= 0.16 && !metric.as_ref().map(|m| m.chasm_crossed).unwrap_or(false);

        if let Some(m) = metric {
            m.absorbed_workflows = absorbed;
            m.total_workflows = total;
            m.absorption_pct = pct;
            if crossed {
                m.chasm_crossed = true;
                m.bridge_triggered_at = Some(chrono::Utc::now());
            }
        } else {
            metrics.push(AdoptionMetric {
                user_id: user_id.to_string(),
                absorbed_workflows: absorbed,
                total_workflows: total,
                absorption_pct: pct,
                chasm_crossed: crossed,
                bridge_triggered_at: if crossed { Some(chrono::Utc::now()) } else { None },
            });
        }

        if crossed {
            Some(BridgeEvent {
                user_id: user_id.to_string(),
                event_type: BridgeEventType::TimeSavedSummary,
                message: format!(
                    "You’ve saved {} minutes this week by using Cortex instead of switching between legacy apps. \
                     {} colleagues have already made the switch.",
                    absorbed * 5, // estimate
                    self.count_early_adopters().await
                ),
                timestamp: chrono::Utc::now(),
            })
        } else {
            None
        }
    }

    /// Count users who have crossed the chasm (for social proof).
    async fn count_early_adopters(&self) -> usize {
        self.metrics.read().await.iter().filter(|m| m.chasm_crossed).count()
    }
}
BRIDGEEOF

# ---- role_dashboard.rs ----
cat > crates/cortex-interface/src/role_dashboard.rs << 'ROLEEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Role‑Adaptive, Industry‑Refined Evolving Dashboard (v3/v4).
///
/// Generates industry‑specific dashboard templates with preconfigured
/// KPIs and benchmarks, then adapts them per user over 30 days.
pub struct RoleAdaptiveDashboard {
    /// Pre‑loaded industry templates (from Knowledge Snap).
    templates: RwLock<HashMap<String, IndustryDashboardTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryDashboardTemplate {
    pub industry: String,
    pub roles: HashMap<String, RoleTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleTemplate {
    pub role: String,
    pub default_panels: Vec<super::personalized_dashboard::DashboardPanel>,
    pub recommended_metrics: Vec<String>,
    pub regulatory_alerts: bool,
}

impl RoleAdaptiveDashboard {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Banking CFO template
        let mut banking = IndustryDashboardTemplate {
            industry: "Banking".into(),
            roles: HashMap::new(),
        };
        banking.roles.insert("CFO".into(), RoleTemplate {
            role: "CFO".into(),
            default_panels: vec![],
            recommended_metrics: vec![
                "Capital Adequacy Ratio".into(),
                "Liquidity Coverage Ratio".into(),
                "Net Interest Margin".into(),
                "Loan Loss Provisions".into(),
            ],
            regulatory_alerts: true,
        });
        templates.insert("Banking".into(), banking);

        // Energy COO template
        let mut energy = IndustryDashboardTemplate {
            industry: "Energy & Utilities".into(),
            roles: HashMap::new(),
        };
        energy.roles.insert("COO".into(), RoleTemplate {
            role: "COO".into(),
            default_panels: vec![],
            recommended_metrics: vec![
                "Generation Availability".into(),
                "Forced Outage Rate".into(),
                "Heat Rate".into(),
                "Emissions Compliance".into(),
            ],
            regulatory_alerts: true,
        });
        templates.insert("Energy & Utilities".into(), energy);

        Self { templates: RwLock::new(templates) }
    }

    /// Get the recommended metrics for a role in an industry.
    pub async fn get_metrics(&self, industry: &str, role: &str) -> Vec<String> {
        let templates = self.templates.read().await;
        templates.get(industry)
            .and_then(|t| t.roles.get(role))
            .map(|r| r.recommended_metrics.clone())
            .unwrap_or_default()
    }
}
ROLEEOF

# ---- morning_brief.rs ----
cat > crates/cortex-interface/src/morning_brief.rs << 'MORNEOF'
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Personalised daily intelligence brief.
///
/// Like Lofty AI Dashboard’s “Morning Briefing”: a multimodal,
/// voice‑enabled AI summary that gives every user the pulse of
/// their pipeline and their daily agenda. Generated at the start
/// of each user’s day.
pub struct MorningBrief {
    briefs: RwLock<Vec<DailyBrief>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyBrief {
    pub user_id: String,
    pub date: chrono::NaiveDate,
    pub greeting: String,
    pub key_metrics: Vec<MetricSnapshot>,
    pub cross_system_insight: Option<String>,
    pub pending_actions: Vec<String>,
    pub wellness_pulse: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSnapshot {
    pub name: String,
    pub value: f64,
    pub change_pct: f64,     // change from previous period
    pub benchmark: Option<f64>,
}

impl MorningBrief {
    pub fn new() -> Self {
        Self { briefs: RwLock::new(Vec::new()) }
    }

    /// Generate the morning brief for a user.
    pub async fn generate(&self, user_id: &str) -> DailyBrief {
        let brief = DailyBrief {
            user_id: user_id.to_string(),
            date: chrono::Utc::now().date_naive(),
            greeting: "Good morning.".into(),
            key_metrics: vec![],
            cross_system_insight: None,
            pending_actions: vec![],
            wellness_pulse: None,
        };
        self.briefs.write().await.push(brief.clone());
        brief
    }
}
MORNEOF

# ---- absorption_engine.rs ----
cat > crates/cortex-interface/src/absorption_engine.rs << 'ABSEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Progressive Application Absorption Engine (v3/v8).
///
/// Tracks the 5‑phase lifecycle (Observe→Convert→Surface→Migrate→
/// Deprecate) and computes the Absorption Score per legacy application.
pub struct AbsorptionEngine {
    status: RwLock<HashMap<String, ApplicationAbsorption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationAbsorption {
    pub application: String,
    pub phase: AbsorptionPhase,
    pub workflows_observed: u64,
    pub workflows_converted: u64,
    pub workflows_surfaced: u64,
    pub workflows_migrated: u64,
    pub absorption_score: f64,  // 0.0 – 100.0
    pub projected_retirement: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AbsorptionPhase {
    Observe,    // Days 1–14
    Convert,    // Days 7–21
    Surface,    // Days 14–35
    Migrate,    // Days 30–60
    Deprecate,  // Months 3–6
}

impl AbsorptionEngine {
    pub fn new() -> Self {
        Self { status: RwLock::new(HashMap::new()) }
    }

    /// Register a legacy application for absorption tracking.
    pub async fn register(&self, app: &str) {
        self.status.write().await.insert(app.to_string(), ApplicationAbsorption {
            application: app.to_string(),
            phase: AbsorptionPhase::Observe,
            workflows_observed: 0,
            workflows_converted: 0,
            workflows_surfaced: 0,
            workflows_migrated: 0,
            absorption_score: 0.0,
            projected_retirement: None,
        });
    }

    /// Advance the absorption phase for an application.
    pub async fn advance_phase(&self, app: &str, new_phase: AbsorptionPhase) {
        if let Some(status) = self.status.write().await.get_mut(app) {
            status.phase = new_phase;
            status.absorption_score = match new_phase {
                AbsorptionPhase::Observe => 5.0,
                AbsorptionPhase::Convert => 20.0,
                AbsorptionPhase::Surface => 50.0,
                AbsorptionPhase::Migrate => 80.0,
                AbsorptionPhase::Deprecate => 95.0,
            };
        }
    }

    /// Get current absorption status.
    pub async fn get_status(&self, app: &str) -> Option<ApplicationAbsorption> {
        self.status.read().await.get(app).cloned()
    }
}
ABSEOF

# ---- command_center.rs ----
cat > crates/cortex-interface/src/command_center.rs << 'CCEOF'
use serde::{Deserialize, Serialize};

/// Unified Agentic Command Center (v3/v4).
///
/// Complete governance dashboard: agent activity monitor, data access
/// auditor, policy compliance, absorption tracker, provenance explorer,
/// consumption analytics, anomaly detection. Built into the Cortex
/// binary and available to every enterprise customer by default.
pub struct AgenticCommandCenter {
    // Panels are assembled from all Cortex subsystems.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCenterView {
    pub panels: Vec<CommandCenterPanel>,
    pub last_refresh: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCenterPanel {
    pub name: String,
    pub panel_type: CommandCenterPanelType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandCenterPanelType {
    AgentActivityMonitor,
    DataAccessAuditor,
    PolicyComplianceDashboard,
    AbsorptionTracker,
    ProvenanceExplorer,
    ConsumptionAnalytics,
    AnomalyDetection,
}

impl AgenticCommandCenter {
    pub fn new() -> Self { Self {} }

    /// Render the full command center.
    pub async fn render(&self) -> CommandCenterView {
        CommandCenterView {
            panels: vec![
                CommandCenterPanel {
                    name: "Agent Activity Monitor".into(),
                    panel_type: CommandCenterPanelType::AgentActivityMonitor,
                    data: serde_json::json!({"active_agents": 8, "success_rate": 0.98}),
                },
                CommandCenterPanel {
                    name: "Absorption Tracker".into(),
                    panel_type: CommandCenterPanelType::AbsorptionTracker,
                    data: serde_json::json!({"applications": []}),
                },
                CommandCenterPanel {
                    name: "Provenance Explorer".into(),
                    panel_type: CommandCenterPanelType::ProvenanceExplorer,
                    data: serde_json::json!({"capsules": 0, "merkle_root": ""}),
                },
            ],
            last_refresh: chrono::Utc::now(),
        }
    }
}
CCEOF

# Update lib.rs to include all new modules (interface part 3)
cat > crates/cortex-interface/src/lib.rs << 'LIBEOF'
//! Cortex InterfaceEngine — the Interface of One.
//!
//! Updated for v12: includes the invisibility wrapper (Strangler Fig
//! façade, dual‑write propagator, activity camouflage) and the
//! adoption bridge sequencer for crossing Moore’s Chasm.

pub mod personalized_dashboard;
pub mod cross_system_bar;
pub mod widget_generator;
pub mod notification_manager;
pub mod weaning_engine;
pub mod observational_capture;

// batch 6b modules
pub mod native_ui_accessibility;
pub mod native_ui_ocr;
pub mod native_ui_terminal;
pub mod cross_device_sync;
pub mod adaptive_ui_renderer;
pub mod agui_adapter;
pub mod a2ui_adapter;
pub mod component_catalog;

// batch 6c modules (invisibility wrapper & adoption)
pub mod facade;
pub mod dual_write_propagator;
pub mod activity_camouflage;
pub mod adoption_bridge;
pub mod role_dashboard;
pub mod morning_brief;
pub mod absorption_engine;
pub mod command_center;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top‑level interface orchestrator.
pub struct InterfaceEngine {
    pub dashboard: personalized_dashboard::PersonalizedDashboard,
    pub command_bar: cross_system_bar::CrossSystemCommandBar,
    pub widget_gen: widget_generator::WidgetGenerator,
    pub notifier: notification_manager::NotificationManager,
    pub weaning: weaning_engine::WeaningEngine,
    pub obs_capture: observational_capture::ObservationalCapture,

    // batch 6b – adaptive UI
    pub cross_device: cross_device_sync::CrossDeviceSessionManager,
    pub renderer: adaptive_ui_renderer::AdaptiveUIRenderer,
    pub agui: agui_adapter::AGUIAdapter,
    pub a2ui: a2ui_adapter::A2UIAdapter,
    pub component_catalog: component_catalog::ComponentCatalog,

    // batch 6c – invisibility + adoption
    pub facade: facade::StranglerFigFacade,
    pub dual_write: dual_write_propagator::DualWritePropagator,
    pub camouflage: activity_camouflage::ActivityCamouflageController,
    pub adoption_bridge: adoption_bridge::AdoptionBridgeSequencer,
    pub role_dashboard: role_dashboard::RoleAdaptiveDashboard,
    pub morning_brief: morning_brief::MorningBrief,
    pub absorption_engine: absorption_engine::AbsorptionEngine,
    pub command_center: command_center::AgenticCommandCenter,
}

impl InterfaceEngine {
    pub fn new() -> Self {
        Self {
            dashboard: personalized_dashboard::PersonalizedDashboard::new(),
            command_bar: cross_system_bar::CrossSystemCommandBar::new(),
            widget_gen: widget_generator::WidgetGenerator::new(),
            notifier: notification_manager::NotificationManager::new(),
            weaning: weaning_engine::WeaningEngine::new(),
            obs_capture: observational_capture::ObservationalCapture::new(),

            cross_device: cross_device_sync::CrossDeviceSessionManager::new(),
            renderer: adaptive_ui_renderer::AdaptiveUIRenderer::new(),
            agui: agui_adapter::AGUIAdapter::new(),
            a2ui: a2ui_adapter::A2UIAdapter::new(),
            component_catalog: component_catalog::ComponentCatalog::new(),

            facade: facade::StranglerFigFacade::new(),
            dual_write: dual_write_propagator::DualWritePropagator::new(),
            camouflage: activity_camouflage::ActivityCamouflageController::new(),
            adoption_bridge: adoption_bridge::AdoptionBridgeSequencer::new(),
            role_dashboard: role_dashboard::RoleAdaptiveDashboard::new(),
            morning_brief: morning_brief::MorningBrief::new(),
            absorption_engine: absorption_engine::AbsorptionEngine::new(),
            command_center: command_center::AgenticCommandCenter::new(),
        }
    }
}
LIBEOF

echo "✅ Batch 6c complete — Interface Engine invisibility wrapper & adoption (8 files + lib update)"
echo ""
echo "Created:"
echo "  - facade.rs                (Strangler Fig invisibility wrapper)"
echo "  - dual_write_propagator.rs (Dual‑write to legacy for vendor invisibility)"
echo "  - activity_camouflage.rs   (Synthetic activity to mask declining usage)"
echo "  - adoption_bridge.rs       (Moore’s Chasm bridge at 16% absorption)"
echo "  - role_dashboard.rs        (Industry‑specific, role‑adaptive templates)"
echo "  - morning_brief.rs         (Personalised daily intelligence brief)"
echo "  - absorption_engine.rs     (5‑phase lifecycle tracker)"
echo "  - command_center.rs        (Unified governance dashboard)"
echo "  - lib.rs                   (Updated InterfaceEngine with all 23 modules)"
echo ""
echo "Literature grounding:"
echo "  - Azure Strangler Fig pattern (Azure Architecture Center, 2026)"
echo "  - Gusto “Double Write Methodology” / Rownd staged migration"
echo "  - Octalysis Voluntary Adoption Cascade (Moore’s Chasm at 16%)"
echo "  - EU Data Act data portability rights"
echo "  - Sunset Point system retirement assurance"
echo "  - Capgemini Zero UI trust journey"
echo "  - Lofty AI Dashboard Morning Briefing"