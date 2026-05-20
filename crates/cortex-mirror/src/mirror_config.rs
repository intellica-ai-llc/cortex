use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Declarative YAML configuration for a Mirror pipeline.
///
/// Modeled after Flink CDC 3.6.0’s YAML‑declarative pipeline
/// specification. The Schema Grounding Agent generates these
/// configs automatically from observed field access patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorPipelineConfig {
    pub source: SourceEndpointConfig,
    pub target: TargetEndpointConfig,
    pub tables: Vec<TableMappingConfig>,
    pub mode: PipelineMode,
    pub backpressure: BackpressureConfig,
    pub validation: ValidationConfig,
    pub camouflage: Option<CamouflageConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceEndpointConfig {
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub credentials: CredentialsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsConfig {
    pub vault_path: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetEndpointConfig {
    pub tracedb_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMappingConfig {
    pub source_schema: String,
    pub source_table: String,
    pub target_table: String,
    pub columns: Vec<String>,
    pub primary_key: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineMode {
    Streaming,
    MicroBatch,
    BulkBatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackpressureConfig {
    pub max_credits: i64,
    pub sustained_limit_seconds: u64,
    pub max_heap_pct: i64,
    pub max_compaction_debt_gb: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub enabled: bool,
    pub sample_fraction: f64,
    pub required_match_rate: f64,
    pub stabilisation_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamouflageConfig {
    pub min_sessions: u32,
    pub min_daily_queries: u32,
}

impl Default for MirrorPipelineConfig {
    fn default() -> Self {
        Self {
            source: SourceEndpointConfig {
                db_type: "postgresql".into(),
                host: "localhost".into(),
                port: 5432,
                database: "source".into(),
                credentials: CredentialsConfig { vault_path: None, env_prefix: Some("SOURCE_".into()) },
            },
            target: TargetEndpointConfig {
                tracedb_url: "postgresql://localhost/cortex".into(),
            },
            tables: vec![],
            mode: PipelineMode::Streaming,
            backpressure: BackpressureConfig {
                max_credits: 100_000,
                sustained_limit_seconds: 30,
                max_heap_pct: 85,
                max_compaction_debt_gb: 20,
            },
            validation: ValidationConfig {
                enabled: true,
                sample_fraction: 0.05,
                required_match_rate: 0.9999,
                stabilisation_seconds: 300,
            },
            camouflage: None,
        }
    }
}

impl MirrorPipelineConfig {
    /// Load from a YAML file.
    pub fn from_yaml(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Cannot read {}: {}", path, e))?;
        serde_yaml::from_str(&content)
            .map_err(|e| format!("Invalid YAML: {}", e))
    }

    /// Save to a YAML file.
    pub fn to_yaml(&self, path: &str) -> Result<(), String> {
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| format!("Serialisation error: {}", e))?;
        std::fs::write(path, yaml)
            .map_err(|e| format!("Cannot write {}: {}", path, e))
    }
}
