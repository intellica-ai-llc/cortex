use crate::experiment_trait::{ValidatableExperiment, ExperimentError};
use crate::lifecycle_scheduler::LifecycleStage;
use std::collections::HashMap;

/// Registry of all ValidatableExperiments.
///
/// Supports lookup by experiment_id and filtering by lifecycle stage.
/// New experiments are registered at startup via `register()`.
pub struct BenchmarkRegistry {
    experiments: HashMap<String, Box<dyn ValidatableExperiment>>,
}

impl BenchmarkRegistry {
    pub fn new() -> Self {
        Self { experiments: HashMap::new() }
    }

    /// Register an experiment (called during initialisation).
    pub fn register(&mut self, experiment: Box<dyn ValidatableExperiment>) {
        self.experiments.insert(experiment.experiment_id().to_string(), experiment);
    }

    /// Look up an experiment by ID.
    pub fn get(&self, id: &str) -> Result<&dyn ValidatableExperiment, ExperimentError> {
        self.experiments.get(id)
            .map(|e| e.as_ref())
            .ok_or_else(|| ExperimentError::NotFound(id.to_string()))
    }

    /// List all registered experiment IDs.
    pub fn all_ids(&self) -> Vec<&String> {
        self.experiments.keys().collect()
    }

    /// Get all experiments at a given lifecycle stage.
    pub fn by_stage(&self, stage: LifecycleStage) -> Vec<String> {
        self.experiments.iter()
            .filter(|(_, e)| e.lifecycle_stage() == stage)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Number of registered experiments.
    pub fn len(&self) -> usize { self.experiments.len() }
}
