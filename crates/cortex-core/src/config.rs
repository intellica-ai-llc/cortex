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
