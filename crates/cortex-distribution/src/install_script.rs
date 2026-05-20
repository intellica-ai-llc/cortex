use serde::{Deserialize, Serialize};

/// curl | bash installer generation.
///
/// Generates a single‑command online installer and an offline
/// installation script for air‑gapped environments.
pub struct InstallScript;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerConfig {
    pub version: String,
    pub install_path: String,      // "/opt/cortex"
    pub database_url: String,
    pub license_path: Option<String>,
    pub mode: InstallMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallMode { Online, Offline, BYOC }

impl InstallScript {
    pub fn new() -> Self { Self }

    /// Generate the online installer script.
    pub fn generate_online_installer(&self, config: &InstallerConfig) -> String {
        format!(
            r#"#!/bin/bash
set -e
echo "Installing Intellecta Cortex v{}..."
mkdir -p {}
curl -fsSL https://releases.intellica.io/cortex/{}/cortex-linux-amd64 -o {}/cortex
chmod +x {}/cortex
{} cortex init --license {}
{} cortex serve
echo "Cortex installed successfully."
"#,
            config.version, config.install_path, config.version,
            config.install_path, config.install_path,
            config.install_path, config.license_path.as_deref().unwrap_or(""),
            config.install_path,
        )
    }

    /// Generate the offline (air‑gapped) installer script.
    pub fn generate_offline_installer(&self, config: &InstallerConfig) -> String {
        format!(
            r#"#!/bin/bash
set -e
echo "Installing Intellecta Cortex v{} (offline)..."
mkdir -p {}
tar -xzf ./cortex-offline.tar.gz -C {}
{} cortex init --license {} --offline
echo "Cortex installed successfully (offline mode)."
"#,
            config.version, config.install_path, config.install_path,
            config.install_path, config.license_path.as_deref().unwrap_or(""),
        )
    }
}
