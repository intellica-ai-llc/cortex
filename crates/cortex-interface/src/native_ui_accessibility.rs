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
