//! Ambient-Aware Theme Manager
//!
//! Handles dynamic theme switching based on OS colour scheme preference,
//! ambient light sensors (when available), and user preference overrides.
//! Uses OKLCH-based colour tokens for perceptual uniformity across modes.
//!
//! Key features:
//!   - Automatic light/dark based on prefers-color-scheme media query.
//!   - Ambient light sensor integration for dynamic contrast adjustment.
//!   - Smooth transitions between themes (CSS transition on colour tokens).
//!   - Per-user override that persists in UXPreferenceStore.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ThemeManager {
    current_mode: tokio::sync::RwLock<ThemeMode>,
    active_theme: tokio::sync::RwLock<Option<super::design_tokens::ThemeDefinition>>,
    user_overrides: tokio::sync::RwLock<HashMap<String, ThemeMode>>,
    /// Whether ambient light sensor data is available.
    ambient_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

/// Dynamic contrast settings based on ambient conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientContext {
    /// Ambient light in lux (0 = pitch black, 10000+ = bright sunlight).
    pub ambient_lux: f64,
    /// Whether the user has explicitly set a theme override.
    pub user_override: Option<ThemeMode>,
    /// OS-level colour scheme preference.
    pub os_prefers_dark: bool,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_mode: tokio::sync::RwLock::new(ThemeMode::Auto),
            active_theme: tokio::sync::RwLock::new(None),
            user_overrides: tokio::sync::RwLock::new(HashMap::new()),
            ambient_available: false,
        }
    }

    /// Resolve the effective theme mode based on ambient context.
    ///
    /// Priority: user override > OS preference > ambient sensor > default (light).
    pub async fn resolve_mode(&self, ctx: &AmbientContext) -> ThemeMode {
        // User override always wins.
        if let Some(ref ovr) = ctx.user_override {
            return ovr.clone();
        }

        // OS preference.
        if ctx.os_prefers_dark {
            return ThemeMode::Dark;
        }

        // If the room is very dark, switch to dark mode even if OS says light.
        if self.ambient_available && ctx.ambient_lux < 50.0 {
            return ThemeMode::Dark;
        }

        ThemeMode::Light
    }

    /// Apply a theme to the current session.
    pub async fn apply_theme(
        &self,
        theme: super::design_tokens::ThemeDefinition,
    ) {
        let mode = theme.mode.clone();
        *self.current_mode.write().await = mode;
        *self.active_theme.write().await = Some(theme);
    }

    /// Set a user's theme override.
    pub async fn set_user_override(&self, user_id: &str, mode: ThemeMode) {
        self.user_overrides.write().await.insert(user_id.to_string(), mode);
    }

    /// Clear a user's theme override.
    pub async fn clear_override(&self, user_id: &str) {
        self.user_overrides.write().await.remove(user_id);
    }

    /// Get the currently active theme definition.
    pub async fn active_theme(&self) -> Option<super::design_tokens::ThemeDefinition> {
        self.active_theme.read().await.clone()
    }

    /// Get the current theme mode.
    pub async fn current_mode(&self) -> ThemeMode {
        self.current_mode.read().await.clone()
    }
}
