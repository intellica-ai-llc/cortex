use cortex_validate::experiment_trait::ExperimentResult;
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait BenchmarkAdapter: Send + Sync {
    fn benchmark_name(&self) -> &str;
    fn benchmark_version(&self) -> &str;
    async fn configure(&self, params: &serde_json::Value) -> Result<(), BenchmarkError>;
    async fn execute(&self) -> Result<BenchmarkOutput, BenchmarkError>;
    fn parse_results(&self, output: BenchmarkOutput) -> ExperimentResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkOutput {
    pub benchmark: String,
    pub raw_results: serde_json::Value,
    pub execution_time_ms: u64,
    pub exit_code: i32,
}

#[derive(Debug, thiserror::Error)]
pub enum BenchmarkError {
    #[error("Configuration failed: {0}")]
    ConfigError(String),
    #[error("Execution failed: {0}")]
    ExecutionError(String),
    #[error("Parsing failed: {0}")]
    ParseError(String),
}
