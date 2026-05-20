use serde::{Deserialize, Serialize};

/// Tool Call Scaler — validates scaling to 2048+ tool calls.
///
/// IterResearch (ICLR 2026): "Extends to 2048 interactions with
/// significant performance improvement (from 3.5% to 42.5% on
/// BrowseComp). The agent demonstrates that performance does not
/// degrade with iteration count — it actually improves as the
/// evolving report becomes richer."
pub struct ToolCallScaler {
    /// Maximum tool calls before forced termination.
    max_tool_calls: u64,
    /// Performance tracking over iterations.
    metrics: tokio::sync::Mutex<Vec<IterationMetric>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationMetric {
    pub iteration: u64,
    pub tokens_used: u64,
    pub report_quality_estimate: f64,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingReport {
    pub total_iterations: u64,
    pub tokens_used: u64,
    pub avg_tokens_per_iteration: f64,
    pub quality_trend: String,   // "improving", "stable", "declining"
}

impl ToolCallScaler {
    pub fn new() -> Self {
        Self { max_tool_calls: 2048, metrics: tokio::sync::Mutex::new(Vec::new()) }
    }

    /// Record an iteration for scaling analysis.
    pub async fn record(&self, iteration: u64, tokens: u64, quality: f64, latency_ms: u64) {
        self.metrics.lock().await.push(IterationMetric {
            iteration, tokens_used: tokens, report_quality_estimate: quality, latency_ms,
        });
    }

    /// Generate a scaling report.
    pub async fn report(&self) -> ScalingReport {
        let metrics = self.metrics.lock().await;
        if metrics.is_empty() {
            return ScalingReport {
                total_iterations: 0, tokens_used: 0,
                avg_tokens_per_iteration: 0.0, quality_trend: "stable".into(),
            };
        }

        let total: u64 = metrics.iter().map(|m| m.tokens_used).sum();
        let avg_tokens = total as f64 / metrics.len() as f64;

        // Quality trend: compare first half vs second half.
        let mid = metrics.len() / 2;
        let first_half_quality: f64 = metrics[..mid].iter().map(|m| m.report_quality_estimate).sum::<f64>() / mid as f64;
        let second_half_quality: f64 = metrics[mid..].iter().map(|m| m.report_quality_estimate).sum::<f64>() / (metrics.len() - mid) as f64;

        let quality_trend = if second_half_quality > first_half_quality + 0.05 {
            "improving"
        } else if second_half_quality < first_half_quality - 0.05 {
            "declining"
        } else {
            "stable"
        };

        ScalingReport {
            total_iterations: metrics.len() as u64,
            tokens_used: total,
            avg_tokens_per_iteration: avg_tokens,
            quality_trend: quality_trend.into(),
        }
    }

    /// Check if the maximum tool call limit has been reached.
    pub fn is_at_limit(&self, current: u64) -> bool {
        current >= self.max_tool_calls
    }
}
