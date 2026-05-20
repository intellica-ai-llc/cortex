use crate::domain_registry::ResearchDomain;
use crate::lifecycle_scheduler::LifecycleStage;
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};

/// The universal experiment trait — every experiment implements this.
#[async_trait::async_trait]
pub trait ValidatableExperiment: Send + Sync {
    fn experiment_id(&self) -> &str;
    fn name(&self) -> &str;
    fn domain(&self) -> ResearchDomain;
    fn lifecycle_stage(&self) -> LifecycleStage;

    /// Natural-language description for One-Eval NL2Bench matching.
    fn nl_description(&self) -> &str;

    /// Data sources this experiment needs from Cortex subsystems.
    fn required_data(&self) -> Vec<DataSourceSpec>;

    /// Parameters the user can configure.
    fn configurable_parameters(&self) -> Vec<ExperimentParameter>;

    /// Execute the experiment using extracted data.
    async fn execute(
        &self,
        data: ExperimentData,
        params: serde_json::Value,
    ) -> Result<ExperimentResult, ExperimentError>;

    /// Compute primary metrics from raw results.
    fn compute_metrics(&self, result: &ExperimentResult) -> Vec<MetricValue>;

    /// Generate visualisation specs (Vega-Lite JSON).
    fn visualizations(&self, result: &ExperimentResult) -> Vec<VegaLiteSpec>;
}

/// Specifies which data to extract from which Cortex subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceSpec {
    pub subsystem: DataSubsystem,
    pub columns: Vec<String>,
    pub filter: Option<String>,
    pub time_range_minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataSubsystem {
    /// TraceDB decision_traces table.
    DecisionTraces,
    /// Mirrored sync state per source (mirror_sync_state).
    MirrorSyncState,
    /// CDC append log entries.
    CdcAppendLog,
    /// Provenance capsules.
    ProvenanceCapsules,
    /// Agent council mission logs.
    CouncilMissions,
    /// Absorption branches.
    AbsorptionBranches,
    /// Absorbed fields with fidelity scores.
    AbsorbedFields,
    /// Gateway tool-call traces.
    GatewayToolCalls,
    /// Pulse wellness scores.
    PulseScores,
    /// Generated UI validation results.
    GenUIValidation,
    /// Mobile sync states.
    MobileSyncStates,
}

/// Experiment parameter definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub default: serde_json::Value,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Integer,
    Float,
    Boolean,
    String,
    Duration,
}

/// Aggregated data provided to an experiment.
#[derive(Debug, Clone)]
pub struct ExperimentData {
    pub dataframes: std::collections::HashMap<String, DataFrame>,
    pub metadata: ExperimentMetadata,
}

#[derive(Debug, Clone)]
pub struct ExperimentMetadata {
    pub extracted_at: chrono::DateTime<chrono::Utc>,
    pub total_rows: u64,
    pub subsystems_queried: Vec<String>,
}

/// Raw results from experiment execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub status: ExperimentStatus,
    pub raw_metrics: serde_json::Value,
    pub sample_size: u64,
    pub execution_time_ms: u64,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExperimentStatus { Success, PartialSuccess { failures: u64 }, Failed { reason: String } }

/// A computed metric with statistical detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub ci_95_lower: Option<f64>,
    pub ci_95_upper: Option<f64>,
    pub effect_size: Option<EffectSize>,
    pub p_value: Option<f64>,
    pub interpretation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectSize {
    pub method: String,
    pub value: f64,
    pub interpretation: String,
}

/// Vega-Lite visualisation specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VegaLiteSpec {
    pub title: String,
    pub chart_type: String,
    pub spec: serde_json::Value,
    pub description: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ExperimentError {
    #[error("Experiment not found: {0}")]
    NotFound(String),
    #[error("Data extraction failed: {0}")]
    ExtractionFailed(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
}
