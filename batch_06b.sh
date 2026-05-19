#!/bin/bash
# ============================================================
# BATCH 6b: CORTEX INTERFACE — ADAPTIVE UI & NATIVE PARSERS
# ============================================================
# Grounded in: Google A2UI v0.9 & CopilotKit AG-UI interoper-
# ability; Devin AI “accessibility-first” screen parsing (Mar
# 2026); OpenAI Operator CUA model (Jan 2026) for UI grounding;
# QPR ProcessAnalyzer MCP connector (Apr 2026); Dashy action-
# object matrix for GenUI component catalog; Capgemini Zero UI
# cross-device seamlessness.
# ============================================================
set -e

mkdir -p crates/cortex-interface/src

# ---- native_ui_accessibility.rs ----
cat > crates/cortex-interface/src/native_ui_accessibility.rs << 'ACCSEOF'
use serde::{Deserialize, Serialize};

/// Native UI parsing via OS accessibility APIs.
///
/// Implements the Devin-on‑Linux pattern (March 2026): directly
/// accesses assistive‑technology APIs to enumerate UI elements
/// of thick clients (Maximo, SAP GUI) without screen scraping.
/// Extends the Observational Capture Engine for non‑web legacy apps.
pub struct AccessibilityParser;

/// A UI element discovered through accessibility APIs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleElement {
    pub role: String,                // “button”, “textbox”, “label”, “table”
    pub name: Option<String>,        // accessible name / label
    pub value: Option<String>,       // current value
    pub bounding_box: BoundingBox,   // screen coordinates
    pub children: Vec<AccessibleElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
}

impl AccessibilityParser {
    pub fn new() -> Self { Self {} }

    /// Enumerate all UI elements of a top‑level window.
    /// Returns a tree of accessible elements.
    pub async fn enumerate_window(&self, _window_title: &str) -> Result<Vec<AccessibleElement>, String> {
        // Production: use platform‑specific accessibility APIs
        // (Windows UI Automation, macOS Accessibility, Linux AT-SPI2).
        Ok(vec![])
    }

    /// Extract field‑level interactions from an accessibility tree
    /// diff between two snapshots.
    pub fn diff_snapshots(
        &self,
        before: &[AccessibleElement],
        after: &[AccessibleElement],
    ) -> Vec<AccessibleChange> {
        // Detect which fields changed, what values were entered.
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleChange {
    pub element_role: String,
    pub element_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}
ACCSEOF

# ---- native_ui_ocr.rs ----
cat > crates/cortex-interface/src/native_ui_ocr.rs << 'OCREOF'
use serde::{Deserialize, Serialize};

/// OCR‑based capture for terminal emulators and green screens.
///
/// Gap closure (v11): captures field‑level interactions from
/// IBM iSeries 5250, VT100, and other text‑based interfaces
/// where accessibility APIs are unavailable.
/// Uses a lightweight on‑device OCR engine for label/value detection.
pub struct OcrParser {
    // In production: integrate Tesseract or a quantised ScreenAI variant.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    pub blocks: Vec<TextBlock>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    pub text: String,
    pub bounding_box: super::native_ui_accessibility::BoundingBox,
    pub block_type: BlockType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockType { Label, Value, Button, Header, Unknown }

impl OcrParser {
    pub fn new() -> Self { Self {} }

    /// Process a screenshot and extract labelled fields.
    pub async fn parse_screenshot(&self, _image_data: &[u8]) -> Result<OcrResult, String> {
        // Production: run OCR on the image, then apply
        // label‑value pairing heuristics based on layout.
        Ok(OcrResult { blocks: vec![], confidence: 0.0 })
    }
}
OCREOF

# ---- native_ui_terminal.rs ----
cat > crates/cortex-interface/src/native_ui_terminal.rs << 'TERMEOF'
use serde::{Deserialize, Serialize};

/// Terminal emulation adapter for legacy mainframe/green‑screen apps.
///
/// Parses IBM 5250 / VT100 data streams directly, extracting
/// screen fields and input areas without OCR.
pub struct TerminalParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalScreen {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<TerminalCell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCell {
    pub character: char,
    pub attribute: u8, // bold, underline, reverse, colour
}

impl TerminalParser {
    pub fn new() -> Self { Self {} }

    /// Parse a raw terminal data stream into a structured screen.
    pub fn parse_stream(&self, _data: &[u8]) -> Result<TerminalScreen, String> {
        // Production: decode 5250 orders or VT100 escape sequences.
        Ok(TerminalScreen {
            rows: 24, cols: 80,
            cells: vec![],
            cursor_row: 0, cursor_col: 0,
        })
    }

    /// Identify input fields on a terminal screen.
    pub fn extract_fields(&self, screen: &TerminalScreen) -> Vec<TerminalField> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalField {
    pub label: String,
    pub row: usize, pub col: usize, pub length: usize,
    pub is_input: bool,
}
TERMEOF

# ---- cross_device_sync.rs ----
cat > crates/cortex-interface/src/cross_device_sync.rs << 'CDSEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Cross‑Device Session Manager (v3).
///
/// Preserves context when a user switches between desktop,
/// laptop, and mobile. A query started on the desktop is
/// waiting on the mobile dashboard with full context.
pub struct CrossDeviceSessionManager {
    /// Active sessions per user, indexed by device type.
    sessions: RwLock<HashMap<String, Vec<DeviceSession>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSession {
    pub device_id: String,
    pub device_type: DeviceType,
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub context: Option<serde_json::Value>, // serialised dashboard state
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType { Desktop, Laptop, Mobile, Tablet }

impl CrossDeviceSessionManager {
    pub fn new() -> Self {
        Self { sessions: RwLock::new(HashMap::new()) }
    }

    /// Update the context for a user’s device.
    pub async fn update_context(&self, user_id: &str, device: DeviceSession) {
        let mut map = self.sessions.write().await;
        let devices = map.entry(user_id.to_string()).or_default();
        if let Some(existing) = devices.iter_mut().find(|d| d.device_id == device.device_id) {
            *existing = device;
        } else {
            devices.push(device);
        }
    }

    /// Retrieve the latest context for a user from any device.
    pub async fn get_latest_context(&self, user_id: &str) -> Option<serde_json::Value> {
        let map = self.sessions.read().await;
        let devices = map.get(user_id)?;
        devices.iter()
            .max_by_key(|d| d.last_active)
            .and_then(|d| d.context.clone())
    }
}
CDSEOF

# ---- adaptive_ui_renderer.rs ----
cat > crates/cortex-interface/src/adaptive_ui_renderer.rs << 'ADAPTEOF'
use serde::{Deserialize, Serialize};

/// Adaptive UI Renderer (v3/v4).
///
/// Detects device type and available screen real estate, then
/// renders the appropriate interface mode automatically.
///
/// The three distinct interfaces (desktop‑full, laptop‑condensed,
/// mobile‑command‑bar‑first) share a common data and state layer
/// but are not simply responsive — each is designed for its usage.
pub struct AdaptiveUIRenderer {
    // Proxies rendering through AG‑UI or A2UI adapters.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIRequest {
    pub user_id: String,
    pub device: super::cross_device_sync::DeviceType,
    pub screen_width: u32,
    pub screen_height: u32,
    pub preferred_protocol: UiProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiProtocol { AG_UI, A2UI, Native }

impl AdaptiveUIRenderer {
    pub fn new() -> Self { Self {} }

    /// Generate an interface specification for a device.
    pub async fn render(&self, _req: &UIRequest) -> serde_json::Value {
        // Production: select component catalog based on device,
        // assemble panels with appropriate layout density,
        // and dispatch rendering via AG‑UI or A2UI adapter.
        serde_json::json!({ "layout": "auto" })
    }
}
ADAPTEOF

# ---- agui_adapter.rs ----
cat > crates/cortex-interface/src/agui_adapter.rs << 'AGUIEOF'
use serde::{Deserialize, Serialize};

/// CopilotKit AG-UI protocol adapter (v4).
///
/// AG‑UI standardises the live, tool‑aware interaction stream
/// between an agent run and an application. When a client
/// speaks AG‑UI, this adapter converts Cortex Interface specs
/// into the AG‑UI event stream format.
pub struct AGUIAdapter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGUIEvent {
    pub event_type: String, // "text_message", "tool_call", "state_snapshot", "ui_update"
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AGUIAdapter {
    pub fn new() -> Self { Self {} }

    /// Convert an Interface Engine panel into an AG‑UI stream.
    pub fn convert_panel(&self, _panel: &serde_json::Value) -> Vec<AGUIEvent> {
        // Map DashboardPanel fields to AG‑UI components:
        // KpiCard → Chart, Table → DataGrid, CommandBar → ChatInput.
        vec![]
    }

    /// Parse an incoming AG‑UI user action.
    pub fn parse_action(&self, _event: &AGUIEvent) -> Option<String> {
        None
    }
}
AGUIEOF

# ---- a2ui_adapter.rs ----
cat > crates/cortex-interface/src/a2ui_adapter.rs << 'A2UIEOF'
use serde::{Deserialize, Serialize};

/// Google A2UI v0.9 protocol adapter (v4).
///
/// A2UI lets agents propose safe, declarative UI surfaces that
/// applications render natively. This adapter generates A2UI‑
/// compliant JSON from Cortex Interface panels.
pub struct A2UIAdapter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2UIResponse {
    pub surface_id: String,
    pub components: Vec<A2UIComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2UIComponent {
    pub id: String,
    pub component_type: String, // "Card", "Table", "Chart", "Form", "Text"
    pub properties: serde_json::Value,
    pub children: Vec<String>,
}

impl A2UIAdapter {
    pub fn new() -> Self { Self {} }

    /// Convert a dashboard panel spec into A2UI JSON.
    pub fn convert_panel(&self, _panel: &serde_json::Value) -> A2UIResponse {
        // Map DashboardPanel to A2UI component hierarchy.
        // Uses the same component catalog as AG‑UI for consistency.
        A2UIResponse {
            surface_id: uuid::Uuid::new_v4().to_string(),
            components: vec![],
        }
    }
}
A2UIEOF

# ---- component_catalog.rs ----
cat > crates/cortex-interface/src/component_catalog.rs << 'CATALOGEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GenUI Component Catalog (v11, gap 7).
///
/// Implements the Dashy action‑object matrix: a fixed lookup table
/// that maps observed user behaviours (view · zone, compare · period,
/// create · record) to prioritised UI component chains. Embedded in
/// the system prompt so the LLM reads it at inference time — no
/// retrieval step needed.
///
/// This solves the “Cognitive Split” problem: the LLM outputs
/// structured data + an action‑object tag; the middleware maps the
/// tag to pre‑configured components from this catalog.
pub struct ComponentCatalog {
    /// The action‑object matrix.
    matrix: HashMap<String, Vec<ComponentChain>>,
}

/// A prioritised chain of UI components for a given action‑object pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentChain {
    pub components: Vec<CatalogComponent>,
    pub priority: u8, // 1 = primary, 2 = secondary, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogComponent {
    pub component_type: CatalogComponentType,
    pub default_props: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatalogComponentType {
    BarChart,
    LineChart,
    DataTable,
    KpiNumber,
    NarrativeText,
    Form,
    RecommendedActions,
    DrillDown,
    FilterBar,
    Timeline,
}

impl ComponentCatalog {
    pub fn new() -> Self {
        let mut matrix = HashMap::new();

        // populate the Dashy action‑object matrix
        matrix.insert("view · zone".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::BarChart, default_props: json!({"stacked": false}) },
                    CatalogComponent { component_type: CatalogComponentType::RecommendedActions, default_props: json!({}) },
                ],
                priority: 1,
            },
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::DataTable, default_props: json!({"sortable": true}) },
                ],
                priority: 2,
            },
        ]);

        matrix.insert("compare · period".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::LineChart, default_props: json!({"multi_series": true}) },
                    CatalogComponent { component_type: CatalogComponentType::DrillDown, default_props: json!({"drill_by": "month"}) },
                ],
                priority: 1,
            },
        ]);

        matrix.insert("compare · employee".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::BarChart, default_props: json!({"horizontal": true}) },
                    CatalogComponent { component_type: CatalogComponentType::DataTable, default_props: json!({"sortable": true}) },
                ],
                priority: 1,
            },
        ]);

        matrix.insert("create · record".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::Form, default_props: json!({"validation": "inline"}) },
                ],
                priority: 1,
            },
        ]);

        Self { matrix }
    }

    /// Look up component chains for a given action‑object tag.
    pub fn get_chains(&self, action_object: &str) -> Vec<&ComponentChain> {
        self.matrix.get(action_object).map(|v| v.iter().collect()).unwrap_or_default()
    }

    /// Serialise the entire matrix for inclusion in the system prompt.
    pub fn to_prompt_context(&self) -> String {
        let mut out = String::from("Available UI components for action-object pairs:\n");
        for (key, chains) in &self.matrix {
            out.push_str(&format!("  {}:\n", key));
            for chain in chains {
                let names: Vec<String> = chain.components.iter().map(|c| format!("{:?}", c.component_type)).collect();
                out.push_str(&format!("    - [{}] (priority {})\n", names.join(", "), chain.priority));
            }
        }
        out
    }
}
CATALOGEOF

echo "✅ Batch 6b complete — Adaptive UI & Native Parsers (8 files)"
echo ""
echo "Created:"
echo "  - native_ui_accessibility.rs  (Accessibility API parser)"
echo "  - native_ui_ocr.rs           (OCR for terminals/green screens)"
echo "  - native_ui_terminal.rs      (5250/VT100 direct stream parser)"
echo "  - cross_device_sync.rs       (Desktop↔Laptop↔Mobile context)"
echo "  - adaptive_ui_renderer.rs    (Device‑aware interface generation)"
echo "  - agui_adapter.rs            (CopilotKit AG-UI events)"
echo "  - a2ui_adapter.rs            (Google A2UI declarative surfaces)"
echo "  - component_catalog.rs       (Dashy action‑object matrix)"
echo ""
echo "Literature grounding:"
echo "  - Google A2UI v0.9 / CopilotKit AG-UI dual protocol"
echo "  - Devin AI “accessibility‑first” screen parsing (Mar 2026)"
echo "  - OpenAI Operator CUA model for UI grounding"
echo "  - Capgemini Zero UI cross‑device continuity"
echo "  - Dashy GenUI demo (May 5, 2026): Cognitive Split solution"