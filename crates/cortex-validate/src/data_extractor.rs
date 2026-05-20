use crate::experiment_trait::{DataSourceSpec, ExperimentData, ExperimentError, ExperimentMetadata};
use polars::prelude::*;
use std::collections::HashMap;

/// Trace-level data extractor — Meta-Harness pattern.
///
/// Based on Meta-Harness (Lee et al., Mar 2026): "Access to execution
/// traces versus access to scores alone produces a 15-point accuracy gap."
/// This extractor pulls raw trace data, not aggregated metrics.
///
/// Uses Polars + Apache Arrow for zero-copy columnar DataFrames.
/// Data flows: TraceDB tables → Arrow record batches → Polars DataFrames.
pub struct TraceDataExtractor;

impl TraceDataExtractor {
    pub fn new() -> Self { Self }

    /// Extract data from multiple Cortex subsystems.
    ///
    /// For each DataSourceSpec, this queries the appropriate TraceDB
    /// table or subsystem API and returns a Polars DataFrame. All
    /// DataFrames share the Arrow memory pool for zero-copy operations.
    pub async fn extract(
        &self,
        specs: &[DataSourceSpec],
    ) -> Result<ExperimentData, ExperimentError> {
        let mut dataframes: HashMap<String, DataFrame> = HashMap::new();
        let mut total_rows = 0u64;
        let mut subsystems = Vec::new();

        for spec in specs {
            let subsystem_name = format!("{:?}", spec.subsystem);
            // In production: query the actual TraceDB tables via sqlx,
            // convert result sets to Arrow record batches,
            // and wrap as Polars DataFrames.
            let df = self.extract_from_subsystem(spec).await?;
            total_rows += df.height() as u64;
            dataframes.insert(subsystem_name.clone(), df);
            subsystems.push(subsystem_name);
        }

        Ok(ExperimentData {
            dataframes,
            metadata: ExperimentMetadata {
                extracted_at: chrono::Utc::now(),
                total_rows,
                subsystems_queried: subsystems,
            },
        })
    }

    async fn extract_from_subsystem(
        &self,
        _spec: &DataSourceSpec,
    ) -> Result<DataFrame, ExperimentError> {
        // In production: sqlx query → Arrow → Polars.
        // For now, return an empty DataFrame with the requested columns.
        let columns: Vec<Series> = _spec.columns.iter()
            .map(|c| Series::new(c.into(), &Vec::<String>::new()))
            .collect();
        DataFrame::new(columns)
            .map_err(|e| ExperimentError::ExtractionFailed(e.to_string()))
    }
}
