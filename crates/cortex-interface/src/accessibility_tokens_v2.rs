//! Accessibility Tokens v2 – WCAG 2.2 AA compliant.
//!
//! WCAG 2.2 (W3C, finalised October 2023, ISO standard 2026) is now the
//! de facto legal benchmark across the EU, UK, and US. Level AA requires
//! 56 total criteria: 32 from Level A plus 24 from Level AA.
//!
//! New WCAG 2.2 criteria (9 added, 1 removed) most relevant to Cortex:
//!   2.4.11 Focus Appearance (AA) – focus indicators ≥2px, perimeter ≥
//!     component perimeter, contrast ≥3:1.
//!   2.4.12 Focus Not Obscured (AA) – sticky elements must not fully hide
//!     the focused element.
//!   2.5.7 Dragging Movements (AA) – any drag must have a pointer alternative.
//!   2.5.8 Target Size Minimum (AA) – interactive targets ≥24×24 CSS pixels.
//!   3.2.6 Consistent Help (A) – contact mechanism in same relative position.
//!   3.3.7 Accessible Authentication (AA) – no cognitive function tests.
//!   3.3.8 Redundant Entry (A) – previously entered info auto‑populated.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityTokensV2 {
    pub wcag_version: String,         // "2.2"
    pub conformance_level: String,     // "AA"
    pub total_criteria: u32,           // 56
    pub focus_ring: FocusRingV2,
    pub target_size: TargetSize,
    pub dragging_alternative: bool,
    pub accessible_authentication: bool,
    pub consistent_help: bool,
    pub reduced_motion: ReducedMotionV2,
    pub keyboard_nav: KeyboardNavV2,
    pub screen_reader: ScreenReaderTokensV2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusRingV2 {
    pub min_thickness_px: u32,       // ≥2px per 2.4.11
    pub min_contrast_ratio: f64,      // ≥3:1 per 2.4.11
    pub surrounds_component: bool,     // perimeter must be ≥ component perimeter
    pub color_token: String,           // OKLCH token, not raw hex
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetSize {
    pub min_size_px: u32,             // 24×24 per 2.5.8
    pub applies_to: Vec<String>,       // "all interactive elements"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReducedMotionV2 {
    pub enabled: bool,
    pub transition_duration_ms: u64,
    pub respects_prefers_reduced_motion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardNavV2 {
    pub focus_not_obscured: bool,      // per 2.4.12
    pub sticky_elements_dismissible: bool,
    pub tab_index_order: Vec<String>,
    pub escape_closes_modal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenReaderTokensV2 {
    pub aria_live_region_id: String,
    pub status_message_role: String,
    pub redundant_entry_autofill: bool,  // per 3.3.8
}

impl AccessibilityTokensV2 {
    pub fn new() -> Self {
        Self {
            wcag_version: "2.2".into(),
            conformance_level: "AA".into(),
            total_criteria: 56,
            focus_ring: FocusRingV2 {
                min_thickness_px: 2,
                min_contrast_ratio: 3.0,
                surrounds_component: true,
                color_token: "oklch(0.55 0.20 264 / 1)".into(),
            },
            target_size: TargetSize {
                min_size_px: 24,
                applies_to: vec!["all interactive elements".into()],
            },
            dragging_alternative: true,
            accessible_authentication: true,
            consistent_help: true,
            reduced_motion: ReducedMotionV2 {
                enabled: false,
                transition_duration_ms: 0,
                respects_prefers_reduced_motion: true,
            },
            keyboard_nav: KeyboardNavV2 {
                focus_not_obscured: true,
                sticky_elements_dismissible: true,
                tab_index_order: vec!["command-bar".into(), "main-content".into()],
                escape_closes_modal: true,
            },
            screen_reader: ScreenReaderTokensV2 {
                aria_live_region_id: "cortex-live-region".into(),
                status_message_role: "status".into(),
                redundant_entry_autofill: true,
            },
        }
    }
}
