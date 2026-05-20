use async_trait::async_trait;
use super::bench_trait::*;

pub struct Adapter;

impl Adapter { pub fn new() -> Self { Self } }

#[async_trait]
impl BenchmarkAdapter for Adapter {
    fn benchmark_name(&self) -> &str { "autoresearch_adapter" }
    fn benchmark_version(&self) -> &str { "1.0" }
    async fn configure(&self, _params: &serde_json::Value) -> Result<(), BenchmarkError> { Ok(()) }
    async fn execute(&self) -> Result<BenchmarkOutput, BenchmarkError> {
        Ok(BenchmarkOutput { benchmark: self.benchmark_name().into(), raw_results: serde_json::json!({}), execution_time_ms: 0, exit_code: 0 })
    }
    fn parse_results(&self, output: BenchmarkOutput) -> cortex_validate::experiment_trait::ExperimentResult {
        cortex_validate::experiment_trait::ExperimentResult {
            experiment_id: self.benchmark_name().into(),
            status: cortex_validate::experiment_trait::ExperimentStatus::Success,
            raw_metrics: output.raw_results,
            sample_size: 0, execution_time_ms: output.execution_time_ms, warnings: vec![],
        }
    }
}
