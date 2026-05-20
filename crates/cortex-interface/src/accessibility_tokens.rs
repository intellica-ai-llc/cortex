//! Accessibility Tokens — Focus-Ring, ARIA, Keyboard-Nav Definitions
//!
//! Defines the WCAG 2.1 AA-mandated design tokens for all interactive
//! components: focus indicators, ARIA attributes, keyboard navigation
//! patterns, screen-reader labels, and reduced-motion preferences.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityTokens {
    pub focus_ring: FocusRing,
    pub reduced_motion: ReducedMotion,
    pub keyboard_nav: KeyboardNav,
    pub screen_reader: ScreenReaderTokens,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusRing {
    pub width_px: u32,
    pub color: String,
    pub offset_px: u32,
    pub style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReducedMotion {
    pub enabled: bool,
    pub transition_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardNav {
    pub tab_index_order: Vec<String>,
    pub escape_closes_modal: bool,
    pub arrow_keys_navigate_list: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenReaderTokens {
    pub aria_live_region_id: String,
    pub status_message_role: String,
}

impl AccessibilityTokens {
    pub fn new() -> Self {
        Self {
            focus_ring: FocusRing {
                width_px: 3, color: "oklch(0.55 0.20 264 / 1)".into(),
                offset_px: 2, style: "solid".into(),
            },
            reduced_motion: ReducedMotion { enabled: false, transition_duration_ms: 0 },
            keyboard_nav: KeyboardNav {
                tab_index_order: vec!["command-bar".into(), "main-content".into(), "side-panel".into()],
                escape_closes_modal: true, arrow_keys_navigate_list: true,
            },
            screen_reader: ScreenReaderTokens {
                aria_live_region_id: "cortex-live-region".into(),
                status_message_role: "status".into(),
            },
        }
    }
}
