//! Cortex Validate™ — Autonomous Experiment Design, Execution & Analysis.
//!
//! A closed-loop validation engine that:
//!   1. Accepts natural-language experiment descriptions
//!   2. Maps them to registered ValidatableExperiments
//!   3. Pulls trace-level data from all Cortex subsystems
//!   4. Executes against industry benchmarks
//!   5. Computes effect sizes, confidence intervals, significance
//!   6. Produces structured, versioned AnalysisReports
//!
//! Based on AutonomyLens (Agrawal et al., FSE 2026) for closed-loop
//! validation, CIRCLE (Westling et al., Feb 2026) for lifecycle staging,
//! and Valohai (Mar 2026) for reproducible-by-default lineage.

pub mod experiment_trait;
pub mod experiment_designer;
pub mod data_extractor;
pub mod statistical_analyser;
pub mod benchmark_registry;
pub mod lifecycle_scheduler;
pub mod result_aggregator;
pub mod visualization_exporter;
pub mod experiment_lineage;
pub mod domain_registry;

// Built-in experiments for the 12 Cortex research domains
pub mod experiments;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top-level validation orchestrator.
pub struct CortexValidate {
    pub registry: Arc<benchmark_registry::BenchmarkRegistry>,
    pub designer: Arc<experiment_designer::ExperimentDesigner>,
    pub extractor: Arc<data_extractor::TraceDataExtractor>,
    pub analyser: Arc<statistical_analyser::StatisticalAnalyser>,
    pub aggregator: Arc<result_aggregator::ResultAggregator>,
    pub viz_exporter: Arc<visualization_exporter::VisualizationExporter>,
    pub lineage: Arc<experiment_lineage::ExperimentLineage>,
    pub scheduler: Arc<lifecycle_scheduler::LifecycleScheduler>,
    /// Map of all 12 Cortex research domains.
    pub domains: Arc<domain_registry::DomainRegistry>,
    /// History of all experiment runs.
    run_history: RwLock<Vec<ExperimentRun>>,
}

/// A single experiment execution record.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExperimentRun {
    pub run_id: String,
    pub experiment_id: String,
    pub experiment_name: String,
    pub domain: domain_registry::ResearchDomain,
    pub lifecycle_stage: lifecycle_scheduler::LifecycleStage,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: RunStatus,
    pub lineage_id: String,
    pub report_path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum RunStatus { Pending, Running, Completed, Failed { reason: String } }

impl CortexValidate {
    pub fn new() -> Self {
        let mut registry = benchmark_registry::BenchmarkRegistry::new();
        // Register all 12 built-in experiments
        experiments::register_all(&mut registry);
        let registry = Arc::new(registry);

        Self {
            registry: registry.clone(),
            designer: Arc::new(experiment_designer::ExperimentDesigner::new(registry.clone())),
            extractor: Arc::new(data_extractor::TraceDataExtractor::new()),
            analyser: Arc::new(statistical_analyser::StatisticalAnalyser::new()),
            aggregator: Arc::new(result_aggregator::ResultAggregator::new()),
            viz_exporter: Arc::new(visualization_exporter::VisualizationExporter::new()),
            lineage: Arc::new(experiment_lineage::ExperimentLineage::new()),
            scheduler: Arc::new(lifecycle_scheduler::LifecycleScheduler::new()),
            domains: Arc::new(domain_registry::DomainRegistry::new()),
            run_history: RwLock::new(Vec::new()),
        }
    }

    /// Run a single experiment by experiment_id.
    pub async fn run_experiment(
        &self,
        experiment_id: &str,
        params: serde_json::Value,
    ) -> Result<result_aggregator::AnalysisReport, experiment_trait::ExperimentError> {
        let experiment = self.registry.get(experiment_id)?;
        let stage = experiment.lifecycle_stage();

        // 1. Create a lineage record (Valohai pattern).
        let lineage_id = self.lineage.create_record(experiment_id, &params).await;

        // 2. Extract trace-level data from Cortex subsystems.
        let data_specs = experiment.required_data();
        let data = self.extractor.extract(&data_specs).await?;

        // 3. Execute the experiment.
        let start = chrono::Utc::now();
        let result = experiment.execute(data, params.clone()).await?;
        let elapsed = (chrono::Utc::now() - start).num_milliseconds() as u64;

        // 4. Compute primary metrics and statistical analysis.
        let metrics = experiment.compute_metrics(&result);
        let stats = self.analyser.analyse(&result, &metrics);

        // 5. Aggregate into a structured AnalysisReport.
        let report = self.aggregator.aggregate(
            experiment_id, experiment.name(),
            experiment.domain(), stage,
            &result, &metrics, &stats,
            elapsed, &lineage_id,
        )?;

        // 6. Generate visualisation specs.
        let _viz_specs = experiment.visualizations(&result);

        // 7. Record in run history.
        self.run_history.write().await.push(ExperimentRun {
            run_id: uuid::Uuid::new_v4().to_string(),
            experiment_id: experiment_id.to_string(),
            experiment_name: experiment.name().to_string(),
            domain: experiment.domain(),
            lifecycle_stage: stage,
            started_at: start,
            completed_at: Some(chrono::Utc::now()),
            status: RunStatus::Completed,
            lineage_id,
            report_path: None,
        });

        Ok(report)
    }

    /// Run all experiments at a given lifecycle stage.
    pub async fn run_stage(
        &self,
        stage: lifecycle_scheduler::LifecycleStage,
    ) -> Vec<result_aggregator::AnalysisReport> {
        let experiments = self.registry.by_stage(stage);
        let mut reports = Vec::new();
        for exp_id in experiments {
            match self.run_experiment(&exp_id, serde_json::json!({})).await {
                Ok(report) => reports.push(report),
                Err(e) => tracing::error!(exp_id, error = %e, "Experiment failed"),
            }
        }
        reports
    }
}
