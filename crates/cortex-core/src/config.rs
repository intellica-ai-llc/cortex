use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Complete Cortex configuration, loaded from cortex.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub license: LicenseConfig,
    pub database: DatabaseConfig,
}

/// Embedded license fields (extracted from the Ed25519-signed payload).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseConfig {
    pub key: String,
    pub customer: String,
    pub plan: String,
    pub seats: u32,
    pub connectors: String,
    pub features: Vec<String>,
    pub expires: String,
    pub signature: String,
}

/// Database connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    /// Load configuration from a file path, falling back to `cortex.toml`
    /// in the workspace root.
    pub fn load(path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = match path {
            Some(p) => PathBuf::from(p),
            None => {
                // Tests run from the crate directory; production runs from workspace root.
                // Try workspace root first, then crate root.
                let candidates = vec![
                    PathBuf::from("cortex.toml"),
                    PathBuf::from("../../cortex.toml"),
                ];
                let found = candidates
                    .iter()
                    .find(|p| p.exists())
                    .cloned()
                    .ok_or("Cannot find cortex.toml in workspace root or crate root")?;
                found
            }
        };

        let contents = fs::read_to_string(&path)
            .map_err(|e| format!("Cannot read config file '{}': {}", path.display(), e))?;
        let config: Config = toml::from_str(&contents)
            .map_err(|e| format!("Invalid TOML in '{}': {}", path.display(), e))?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_config() {
        let config = Config::load(None).unwrap();
        assert_eq!(config.license.plan, "enterprise");
        assert_eq!(config.license.customer, "acme-corp");
        assert_eq!(config.license.seats, 500);
        assert!(config.license.features.contains(&"council_full".to_string()));
    }
}